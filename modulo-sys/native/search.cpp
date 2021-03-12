#define _UNICODE

#include "common.h"
#include "wx/htmllbox.h"

#include "interop.h"

#include <vector>
#include <memory>
#include <unordered_map>

// Platform-specific styles
#ifdef __WXMSW__
const int SEARCH_BAR_FONT_SIZE = 16;
#endif
#ifdef __WXOSX__
const int SEARCH_BAR_FONT_SIZE = 20;
#endif

// TODO: linux

const wxColour SELECTION_LIGHT_BG = wxColour(164, 210, 253);
const wxColour SELECTION_DARK_BG = wxColour(49, 88, 126);

// https://docs.wxwidgets.org/stable/classwx_frame.html
const long DEFAULT_STYLE = wxSTAY_ON_TOP | wxFRAME_TOOL_WINDOW | wxRESIZE_BORDER;
const int MIN_WIDTH = 500;
const int MIN_HEIGHT = 80;

typedef void (*QueryCallback)(const char * query, void * app, void * data);
typedef void (*ResultCallback)(const char * id, void * data);

SearchMetadata *searchMetadata = nullptr;
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

class ResultListBox : public wxHtmlListBox
{
public:
    ResultListBox() { }
    ResultListBox(wxWindow *parent, bool isDark, const wxWindowID id, const wxPoint& pos, const wxSize& size);
protected:
    // override this method to return data to be shown in the listbox (this is
    // mandatory)
    virtual wxString OnGetItem(size_t n) const wxOVERRIDE;

    // change the appearance by overriding these functions (this is optional)
    virtual void OnDrawBackground(wxDC& dc, const wxRect& rect, size_t n) const wxOVERRIDE;

    bool isDark;
public:
    wxDECLARE_NO_COPY_CLASS(ResultListBox);
    wxDECLARE_DYNAMIC_CLASS(ResultListBox);
};

wxIMPLEMENT_DYNAMIC_CLASS(ResultListBox, wxHtmlListBox);

ResultListBox::ResultListBox(wxWindow *parent, bool isDark, const wxWindowID id, const wxPoint& pos, const wxSize& size)
             : wxHtmlListBox(parent, id, pos, size, 0)
{
    this->isDark = isDark;
    SetMargins(5, 5);
    Refresh();
}

void ResultListBox::OnDrawBackground(wxDC& dc, const wxRect& rect, size_t n) const
{
    dc.SetBrush(wxNullBrush);
    dc.SetPen(wxNullPen);
    if (IsSelected(n)) {
        if (isDark) {
            dc.SetBrush(wxBrush(SELECTION_DARK_BG));
        } else {
            dc.SetBrush(wxBrush(SELECTION_LIGHT_BG));
        }
    } else {
        dc.SetBrush(*wxTRANSPARENT_BRUSH);
    }
    dc.DrawRectangle(0, 0, rect.GetRight(), rect.GetBottom());
}

wxString ResultListBox::OnGetItem(size_t n) const
{
    wxString textColor = isDark ? "white" : "";
    wxString shortcut = (n < 8) ? wxString::Format(wxT("⌥%i"), (int) n+1) : " ";
    return wxString::Format(wxT("<font color='%s'><table width='100%%'><tr><td>%s</td><td align='right'><b>%s</b></td></tr></table></font>"), textColor, wxItems[n], shortcut);
}

class SearchFrame: public wxFrame
{
public:
    SearchFrame(const wxString& title, const wxPoint& pos, const wxSize& size);

    wxPanel *panel;
    wxTextCtrl *searchBar;
    wxStaticBitmap *iconPanel;
    ResultListBox *resultBox;
    void SetItems(SearchItem *items, int itemSize);
private:
    void OnCharEvent(wxKeyEvent& event);
    void OnQueryChange(wxCommandEvent& event);
    void OnItemClickEvent(wxCommandEvent& event);
    void SelectNext();
    void SelectPrevious();
    void Submit();
};

bool SearchApp::OnInit()
{
    SearchFrame *frame = new SearchFrame(searchMetadata->windowTitle, wxPoint(50, 50), wxSize(450, 340) );
    setFrameIcon(searchMetadata->iconPath, frame);
    frame->Show( true );
    SetupWindowStyle(frame);
    Activate(frame);
    return true;
}
SearchFrame::SearchFrame(const wxString& title, const wxPoint& pos, const wxSize& size)
        : wxFrame(NULL, wxID_ANY, title, pos, size, DEFAULT_STYLE)
{
    wxInitAllImageHandlers();

    bool isDark = wxSystemSettings::GetAppearance().IsDark();

    panel = new wxPanel(this, wxID_ANY);
    wxBoxSizer *vbox = new wxBoxSizer(wxVERTICAL);
    panel->SetSizer(vbox);

    wxBoxSizer *topBox = new wxBoxSizer(wxHORIZONTAL);

    wxBitmap bitmap = wxBitmap(wxT("C:\\Users\\fredd\\Insync\\Development\\Espanso\\Images\\icongreensmall.png"), wxBITMAP_TYPE_PNG);
    wxImage image = bitmap.ConvertToImage();
    image.Rescale(32, 32, wxIMAGE_QUALITY_HIGH);
    wxBitmap resizedBitmap = wxBitmap(image, -1);
    iconPanel = new wxStaticBitmap( panel, wxID_ANY, resizedBitmap, wxDefaultPosition, wxSize(32, 32));
    topBox->Add(iconPanel, 0, wxEXPAND | wxALL, 10);

    int textId = NewControlId();
    searchBar = new wxTextCtrl(panel, textId, "", wxDefaultPosition, wxDefaultSize);
    wxFont font = searchBar->GetFont();
    font.SetPointSize(SEARCH_BAR_FONT_SIZE);
    searchBar->SetFont(font);
    topBox->Add(searchBar, 1, wxEXPAND | wxRIGHT | wxUP | wxDOWN, 10);

    vbox->Add(topBox, 1, wxEXPAND);
    
    wxArrayString choices;
    int resultId = NewControlId();
    resultBox = new ResultListBox(panel, isDark, resultId, wxDefaultPosition, wxSize(MIN_WIDTH, MIN_HEIGHT));
    vbox->Add(resultBox, 5, wxEXPAND | wxALL, 0);

    Bind(wxEVT_CHAR_HOOK, &SearchFrame::OnCharEvent, this, wxID_ANY);
    Bind(wxEVT_TEXT, &SearchFrame::OnQueryChange, this, textId);
    Bind(wxEVT_LISTBOX_DCLICK, &SearchFrame::OnItemClickEvent, this, resultId);

    this->SetClientSize(panel->GetBestSize());
    this->CentreOnScreen();

    // Trigger the first data update
    queryCallback("", (void*) this, data);
}

void SearchFrame::OnCharEvent(wxKeyEvent& event) {
    // TODO: here handle the ALT+Num shortcuts
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

void SearchFrame::OnItemClickEvent(wxCommandEvent& event) {
    resultBox->SetSelection(event.GetInt());
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

    if (itemSize > 0) {
        resultBox->SetSelection(0);
    }
    resultBox->RefreshAll();
    resultBox->Refresh();
}

void SearchFrame::SelectNext() {
    if (resultBox->GetItemCount() > 0 && resultBox->GetSelection() != wxNOT_FOUND) {
        int newSelected = 0;
        if (resultBox->GetSelection() < (resultBox->GetItemCount() - 1)) {
            newSelected = resultBox->GetSelection() + 1;
        }
        
        resultBox->SetSelection(newSelected);
    }
}

void SearchFrame::SelectPrevious() {
    if (resultBox->GetItemCount() > 0 && resultBox->GetSelection() != wxNOT_FOUND) {
        int newSelected = resultBox->GetItemCount() - 1;
        if (resultBox->GetSelection() > 0) {
            newSelected = resultBox->GetSelection() - 1;
        }
        
        resultBox->SetSelection(newSelected);
    }
}

void SearchFrame::Submit() {
    if (resultBox->GetItemCount() > 0 && resultBox->GetSelection() != wxNOT_FOUND) {
        long index = resultBox->GetSelection();
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
    
    searchMetadata = _metadata;
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