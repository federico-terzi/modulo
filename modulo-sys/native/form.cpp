#define _UNICODE

#include <wx/wxprec.h>
#ifndef WX_PRECOMP
    #include <wx/wx.h>
#endif

#include "interop.h"

#include <vector>

// https://docs.wxwidgets.org/stable/classwx_frame.html
const long DEFAULT_STYLE = wxSTAY_ON_TOP | wxRESIZE_BORDER | wxCLOSE_BOX | wxCAPTION;

FormMetadata *metadata = nullptr;

class FormApp: public wxApp
{
public:
    virtual bool OnInit();
};
class FormFrame: public wxFrame
{
public:
    FormFrame(const wxString& title, const wxPoint& pos, const wxSize& size);

    wxPanel *panel;
    std::vector<void *> fields;
private:
    // void AddLabel()
    void OnSubmit(wxCommandEvent& event);
};
enum
{
    ID_Submit = 1
};

wxIMPLEMENT_APP_NO_MAIN(FormApp);

bool FormApp::OnInit()
{
    FormFrame *frame = new FormFrame(metadata->windowTitle, wxPoint(50, 50), wxSize(450, 340) );
    frame->Show( true );
    return true;
}
FormFrame::FormFrame(const wxString& title, const wxPoint& pos, const wxSize& size)
        : wxFrame(NULL, wxID_ANY, title, pos, size, DEFAULT_STYLE)
{
    panel = new wxPanel(this, wxID_ANY);
    wxBoxSizer *vbox = new wxBoxSizer(wxVERTICAL);
    panel->SetSizer(vbox);

    for (int field = 0; field < metadata->fieldSize; field++) {
        FieldMetadata meta = metadata->fields[field];

        switch (meta.fieldType) {
            case FieldType::LABEL:
            {
                const LabelMetadata *label_meta = static_cast<const LabelMetadata*>(meta.specific);
                printf("%s\n", label_meta->text);
                // TODO: make a function to change the parent component
                auto label = new wxStaticText(panel, wxID_ANY, label_meta->text, wxDefaultPosition, wxDefaultSize, wxALIGN_CENTRE_HORIZONTAL);
                vbox->Add(label, 1, wxEXPAND | wxALL, 10);
                fields.push_back(label);
                break;
            }
            default:
                // TODO: handle unknown field type
                break;
        }
    }

    //innerPanel = new wxPanel(panel, wxID_ANY);
    //wxBoxSizer *hbox = new wxBoxSizer(wxHORIZONTAL);
    //wxBoxSizer *vbox = new wxBoxSizer(wxVERTICAL);

    //label = new wxStaticText(innerPanel, wxID_ANY, "test label", wxDefaultPosition, wxDefaultSize, wxALIGN_CENTRE_HORIZONTAL);
    //control = new wxTextCtrl(innerPanel, wxID_ANY);
    //control->ChangeValue(metadata->text);
    //button = new wxButton(panel, ID_Submit, "submit");

    
    //hbox->Add(label, 1, wxEXPAND | wxALL, 0);
    //hbox->Add(control, 1, wxEXPAND | wxALL, 0);
    //innerPanel->SetSizer(hbox);

    //vbox->Add(innerPanel, 1, wxEXPAND | wxALL, 10);
    //vbox->Add(button, 1, wxEXPAND | wxALL, 10);
    

    Bind(wxEVT_BUTTON, &FormFrame::OnSubmit, this, ID_Submit);
    // TODO: register ESC click handler: https://forums.wxwidgets.org/viewtopic.php?t=41926

    this->SetClientSize(panel->GetBestSize());
}

void FormFrame::OnSubmit(wxCommandEvent &event) {
    // TODO: collect all form data

    Close(true);
}

extern "C" void interop_show_form(FormMetadata * _metadata) {
    SetProcessDPIAware();
    metadata = _metadata;
    wxEntry(0, nullptr);
}