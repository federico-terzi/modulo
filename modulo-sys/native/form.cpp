#define _UNICODE

#include <wx/wxprec.h>
#ifndef WX_PRECOMP
    #include <wx/wx.h>
#endif

#include "interop.h"

#include <vector>
#include <unordered_map>

// https://docs.wxwidgets.org/stable/classwx_frame.html
const long DEFAULT_STYLE = wxSTAY_ON_TOP | wxRESIZE_BORDER | wxCLOSE_BOX | wxCAPTION;

const int PADDING = 5;

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
    std::unordered_map<const char *, void *> idMap;
    wxButton *submit;
private:
    void AddComponent(wxPanel *parent, wxBoxSizer *sizer, FieldMetadata meta);
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
        AddComponent(panel, vbox, meta);
    }

    //innerPanel = new wxPanel(panel, wxID_ANY);
    //wxBoxSizer *hbox = new wxBoxSizer(wxHORIZONTAL);
    //wxBoxSizer *vbox = new wxBoxSizer(wxVERTICAL);

    //label = new wxStaticText(innerPanel, wxID_ANY, "test label", wxDefaultPosition, wxDefaultSize, wxALIGN_CENTRE_HORIZONTAL);
    //control = new wxTextCtrl(innerPanel, wxID_ANY);
    //control->ChangeValue(metadata->text);
    submit = new wxButton(panel, ID_Submit, "Submit");
    vbox->Add(submit, 1, wxEXPAND | wxALL, PADDING);

    Bind(wxEVT_BUTTON, &FormFrame::OnSubmit, this, ID_Submit);
    // TODO: register ESC click handler: https://forums.wxwidgets.org/viewtopic.php?t=41926

    this->SetClientSize(panel->GetBestSize());
}

void FormFrame::AddComponent(wxPanel *parent, wxBoxSizer *sizer, FieldMetadata meta) {
    switch (meta.fieldType) {
        case FieldType::LABEL:
        {
            const LabelMetadata *labelMeta = static_cast<const LabelMetadata*>(meta.specific);
            auto label = new wxStaticText(parent, wxID_ANY, labelMeta->text, wxDefaultPosition, wxDefaultSize, wxALIGN_CENTRE_HORIZONTAL);
            sizer->Add(label, 1, wxEXPAND | wxALL, PADDING);
            fields.push_back(label);
            break;
        }
        case FieldType::ROW:
        {
            const RowMetadata *rowMeta = static_cast<const RowMetadata*>(meta.specific);

            auto innerPanel = new wxPanel(panel, wxID_ANY);
            wxBoxSizer *hbox = new wxBoxSizer(wxHORIZONTAL);
            innerPanel->SetSizer(hbox);
            sizer->Add(innerPanel, 1, wxEXPAND | wxALL, 0);
            fields.push_back(innerPanel);

            for (int field = 0; field < rowMeta->fieldSize; field++) {
                FieldMetadata innerMeta = rowMeta->fields[field];
                AddComponent(innerPanel, hbox, innerMeta);
            }

            break;
        }
        case FieldType::TEXT:
        {
            const TextMetadata *textMeta = static_cast<const TextMetadata*>(meta.specific);
            auto textControl = new wxTextCtrl(parent, NewControlId());
            textControl->ChangeValue(textMeta->defaultText);
            idMap[meta.id] = textControl;
            sizer->Add(textControl, 1, wxEXPAND | wxALL, PADDING);
            fields.push_back(textControl);
        }
        default:
            // TODO: handle unknown field type
            break;
    }
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