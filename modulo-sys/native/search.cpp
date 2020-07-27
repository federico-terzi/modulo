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

SearchMetadata *metadata = nullptr;

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
private:
    void OnCharEvent(wxKeyEvent& event);
    void OnSubmitBtn(wxCommandEvent& event);
    void SetItems(SearchItem *items, int itemSize);
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

    searchBar = new wxTextCtrl(panel, NewControlId(), "", wxDefaultPosition, wxDefaultSize);
    vbox->Add(searchBar, 1, wxEXPAND | wxALL, 0);
    
    wxArrayString choices;
    resultBox = new wxListBox(panel, NewControlId(), wxDefaultPosition, wxDefaultSize, choices);
    resultBox->SetMinSize(wxSize(MIN_WIDTH, MIN_HEIGHT));
    vbox->Add(resultBox, 5, wxEXPAND | wxALL, 0);

    Bind(wxEVT_CHAR_HOOK, &SearchFrame::OnCharEvent, this, wxID_ANY);

    this->SetClientSize(panel->GetBestSize());
    this->CentreOnScreen();
}

void SearchFrame::OnCharEvent(wxKeyEvent& event) {
    if (event.GetKeyCode() == WXK_ESCAPE) {
        Close(true);
    }else{
        event.Skip();
    }
}

void SearchFrame::SetItems(SearchItem *items, int itemSize) {
    
}

extern "C" void interop_show_search(SearchMetadata * _metadata, void (*callback)(char * query, void *app), void *data) {
    // Setup high DPI support on Windows
    #ifdef __WXMSW__
        SetProcessDPIAware();
    #endif
    
    metadata = _metadata;
    
    wxApp::SetInstance(new SearchApp());
    int argc = 0;
    wxEntry(argc, (char **)nullptr);
}