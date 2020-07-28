#define _UNICODE

#include <wx/wxprec.h>
#ifndef WX_PRECOMP
    #include <wx/wx.h>
#endif

#include "interop.h"

#include <vector>
#include <memory>
#include <unordered_map>

// https://docs.wxwidgets.org/stable/classwx_frame.html
const long DEFAULT_STYLE = wxSTAY_ON_TOP | wxCLOSE_BOX | wxCAPTION;
const int MIN_WIDTH = 500;
const int MIN_HEIGHT = 20;

typedef void (*QueryCallback)(const char * query, void * app, void * data);

SearchMetadata *metadata = nullptr;
QueryCallback queryCallback = nullptr;
void * data = nullptr;

// App Code

class SearchApp: public wxApp
{
public:
    virtual bool OnInit();
};
class SearchFrame: public wxFrame
{
public:
    SearchFrame(const wxString& title, const wxPoint& pos, const wxSize& size);

    wxPanel *panel;
    wxTextCtrl *searchBar;
    wxListBox *resultBox;
    void SetItems(SearchItem *items, int itemSize);
private:
    void OnCharEvent(wxKeyEvent& event);
    void OnQueryChange(wxCommandEvent& event);
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
    resultBox = new wxListBox(panel, NewControlId(), wxDefaultPosition, wxDefaultSize, choices);
    resultBox->SetMinSize(wxSize(MIN_WIDTH, MIN_HEIGHT));
    vbox->Add(resultBox, 5, wxEXPAND | wxALL, 0);

    Bind(wxEVT_CHAR_HOOK, &SearchFrame::OnCharEvent, this, wxID_ANY);
    Bind(wxEVT_TEXT, &SearchFrame::OnQueryChange, this, textId);

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

void SearchFrame::SetItems(SearchItem *items, int itemSize) {
    resultBox->Clear();

    wxClientData ** data = new wxClientData*[itemSize + 1];

    wxArrayString wxItems;
    for (int i = 0; i<itemSize; i++) {
        wxString item = items[i].label;
        wxItems.Add(item);
        
        wxStringClientData * itemData = new wxStringClientData(items[i].id);
        data[i] = itemData;
    }

    resultBox->Set(wxItems, (wxClientData**) data);

    if (itemSize > 0) {
        resultBox->SetSelection(0);
    }
}

void SearchFrame::SelectNext() {
    if (resultBox->GetCount() > 0 && resultBox->GetSelection() != wxNOT_FOUND) {
        if (resultBox->GetSelection() < (resultBox->GetCount() - 1)) {
            resultBox->SetSelection(resultBox->GetSelection() + 1);
        }else{
            resultBox->SetSelection(0);
        }
    }
}

void SearchFrame::SelectPrevious() {
    if (resultBox->GetCount() > 0 && resultBox->GetSelection() != wxNOT_FOUND) {
        if (resultBox->GetSelection() > 0) {
            resultBox->SetSelection(resultBox->GetSelection() - 1);
        }else{
            resultBox->SetSelection(resultBox->GetCount() - 1);
        }
    }
}

void SearchFrame::Submit() {
    if (resultBox->GetCount() > 0 && resultBox->GetSelection() != wxNOT_FOUND) {
        wxStringClientData * selected = (wxStringClientData*) resultBox->GetClientObject(resultBox->GetSelection());
        wxString id = selected->GetData();
    }
}

extern "C" void interop_show_search(SearchMetadata * _metadata, QueryCallback callback, void *_data) {
    // Setup high DPI support on Windows
    #ifdef __WXMSW__
        SetProcessDPIAware();
    #endif
    
    metadata = _metadata;
    queryCallback = callback;
    data = _data;
    
    wxApp::SetInstance(new SearchApp());
    int argc = 0;
    wxEntry(argc, (char **)nullptr);
}

extern "C" void update_items(void * app, SearchItem * items, int itemSize) {
    SearchFrame * frame = (SearchFrame *) app;
    frame->SetItems(items, itemSize);
}