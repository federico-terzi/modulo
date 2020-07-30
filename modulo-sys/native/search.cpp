#define _UNICODE

#include <wx/wxprec.h>
#ifndef WX_PRECOMP
    #include <wx/wx.h>
#endif
#include <wx/listctrl.h>

#include "interop.h"

#include <vector>
#include <memory>
#include <unordered_map>

// https://docs.wxwidgets.org/stable/classwx_frame.html
const long DEFAULT_STYLE = wxSTAY_ON_TOP | wxCLOSE_BOX | wxCAPTION;
const int MIN_WIDTH = 500;
const int MIN_HEIGHT = 20;

typedef void (*QueryCallback)(const char * query, void * app, void * data);
typedef void (*ResultCallback)(const char * id, void * data);

SearchMetadata *metadata = nullptr;
QueryCallback queryCallback = nullptr;
ResultCallback resultCallback = nullptr;
void * data = nullptr;
void * resultData = nullptr;
wxArrayString wxItems;
wxArrayString wxIds;

// App Code

class SearchApp: public wxApp
{
public:
    virtual bool OnInit();
};

class ResultListView: public wxListView
{
public:
    ResultListView(wxWindow *parent,
               const wxWindowID id,
               const wxPoint& pos,
               const wxSize& size,
               long style)
        : wxListView(parent, id, pos, size, style)
        {}
    void RescaleColumns();
private:
    virtual wxString OnGetItemText(long item, long column) const wxOVERRIDE;
};

wxString ResultListView::OnGetItemText(long item, long column) const
{
    return wxItems[item];
}

// Taken from https://groups.google.com/forum/#!topic/wx-users/jOwhl53ES5M
// Used to hide the horizontal scrollbar
void ResultListView::RescaleColumns()
{
    int nWidth, nHeight;
    GetClientSize(&nWidth, &nHeight);
    const int main_col = 0;
    if (GetColumnWidth(main_col) != nWidth )
    {
        #ifdef __WXMSW__
            SetColumnWidth(main_col, nWidth);
        #else
            SetColumnWidth(main_col, nWidth -
wxSystemSettings::GetMetric(wxSYS_HSCROLL_Y)  - 1 );          
        #endif
    }
}

class SearchFrame: public wxFrame
{
public:
    SearchFrame(const wxString& title, const wxPoint& pos, const wxSize& size);

    wxPanel *panel;
    wxTextCtrl *searchBar;
    ResultListView *resultBox;
    void SetItems(SearchItem *items, int itemSize);
private:
    void OnCharEvent(wxKeyEvent& event);
    void OnQueryChange(wxCommandEvent& event);
    void OnItemClickEvent(wxListEvent& event);
    void SelectNext();
    void SelectPrevious();
    void Submit();
};

bool SearchApp::OnInit()
{
    SearchFrame *frame = new SearchFrame(metadata->windowTitle, wxPoint(50, 50), wxSize(450, 340) );
    frame->Show( true );
    return true;
}
SearchFrame::SearchFrame(const wxString& title, const wxPoint& pos, const wxSize& size)
        : wxFrame(NULL, wxID_ANY, title, pos, size, DEFAULT_STYLE)
{
    panel = new wxPanel(this, wxID_ANY);
    wxBoxSizer *vbox = new wxBoxSizer(wxVERTICAL);
    panel->SetSizer(vbox);

    int textId = NewControlId();
    searchBar = new wxTextCtrl(panel, textId, "", wxDefaultPosition, wxDefaultSize);
    vbox->Add(searchBar, 1, wxEXPAND | wxALL, 0);
    
    wxArrayString choices;
    int resultId = NewControlId();
    resultBox = new ResultListView(panel, resultId, wxDefaultPosition, wxSize(MIN_WIDTH, MIN_HEIGHT), wxLC_VIRTUAL | wxLC_REPORT | wxLC_NO_HEADER | wxLC_SINGLE_SEL);
    resultBox->InsertColumn(0, "Results", wxLIST_FORMAT_LEFT, wxLIST_AUTOSIZE_USEHEADER);
    vbox->Add(resultBox, 5, wxEXPAND | wxALL, 0);

    Bind(wxEVT_CHAR_HOOK, &SearchFrame::OnCharEvent, this, wxID_ANY);
    Bind(wxEVT_TEXT, &SearchFrame::OnQueryChange, this, textId);
    Bind(wxEVT_LIST_ITEM_ACTIVATED, &SearchFrame::OnItemClickEvent, this, resultId);

    this->SetClientSize(panel->GetBestSize());
    this->CentreOnScreen();

    // Trigger the first data update
    queryCallback("", (void*) this, data);
}

void SearchFrame::OnCharEvent(wxKeyEvent& event) {
    if (event.GetKeyCode() == WXK_ESCAPE) {
        Close(true);
    }else if(event.GetKeyCode() == WXK_TAB) {
        if (wxGetKeyState(WXK_SHIFT)) {
            SelectPrevious();
        }else{
            SelectNext();
        }
    }else if(event.GetKeyCode() == WXK_DOWN) {
        SelectNext();
    }else if(event.GetKeyCode() == WXK_UP) {
        SelectPrevious();
    }else if (event.GetKeyCode() == WXK_RETURN) {
        Submit();
    }else{
        event.Skip();
    }
}

void SearchFrame::OnQueryChange(wxCommandEvent& event) {
    wxString queryString = searchBar->GetValue();
    const char * query = queryString.ToUTF8();
    queryCallback(query, (void*) this, data);
}

void SearchFrame::OnItemClickEvent(wxListEvent& event) {
    resultBox->Select(event.GetIndex());
    Submit();
}

void SearchFrame::SetItems(SearchItem *items, int itemSize) {
    wxItems.Clear();
    wxIds.Clear();

    for (int i = 0; i<itemSize; i++) {
        wxString item = items[i].label;
        wxItems.Add(item);
        
        wxString id = items[i].id;
        wxIds.Add(id);
    }

    resultBox->SetItemCount(itemSize);
    resultBox->RefreshItems(0, itemSize-1);
    resultBox->RescaleColumns();
    
    if (itemSize > 0) {
        resultBox->Select(0);
        resultBox->EnsureVisible(0);
    }
}

void SearchFrame::SelectNext() {
    if (resultBox->GetItemCount() > 0 && resultBox->GetFirstSelected() != wxNOT_FOUND) {
        int newSelected = 0;
        if (resultBox->GetFirstSelected() < (resultBox->GetItemCount() - 1)) {
            newSelected = resultBox->GetFirstSelected() + 1;
        }
        
        resultBox->Select(newSelected);
        resultBox->EnsureVisible(newSelected);
    }
}

void SearchFrame::SelectPrevious() {
    if (resultBox->GetItemCount() > 0 && resultBox->GetFirstSelected() != wxNOT_FOUND) {
        int newSelected = resultBox->GetItemCount() - 1;
        if (resultBox->GetFirstSelected() > 0) {
            newSelected = resultBox->GetFirstSelected() - 1;
        }
        
        resultBox->Select(newSelected);
        resultBox->EnsureVisible(newSelected);
    }
}

void SearchFrame::Submit() {
    if (resultBox->GetItemCount() > 0 && resultBox->GetFirstSelected() != wxNOT_FOUND) {
        long index = resultBox->GetFirstSelected();
        wxString id = wxIds[index];
        if (resultCallback) {
            resultCallback(id.ToUTF8(), resultData);
        }

        Close(true);
    }
}

extern "C" void interop_show_search(SearchMetadata * _metadata, QueryCallback _queryCallback, void *_data, ResultCallback _resultCallback, void *_resultData) {
    // Setup high DPI support on Windows
    #ifdef __WXMSW__
        SetProcessDPIAware();
    #endif
    
    metadata = _metadata;
    queryCallback = _queryCallback;
    resultCallback = _resultCallback;
    data = _data;
    resultData = _resultData;
    
    wxApp::SetInstance(new SearchApp());
    int argc = 0;
    wxEntry(argc, (char **)nullptr);
}

extern "C" void update_items(void * app, SearchItem * items, int itemSize) {
    SearchFrame * frame = (SearchFrame *) app;
    frame->SetItems(items, itemSize);
}