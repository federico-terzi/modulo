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
const long DEFAULT_STYLE = wxSTAY_ON_TOP | wxRESIZE_BORDER | wxCLOSE_BOX | wxCAPTION;

const int PADDING = 5;

FormMetadata *metadata = nullptr;
std::vector<ValuePair> values;

// Field Wrappers

class FieldWrapper {
public:
    virtual wxString getValue() = 0;
};

class TextFieldWrapper {
    wxTextCtrl * control;
public:
    explicit TextFieldWrapper(wxTextCtrl * control): control(control) {}

    virtual wxString getValue() {
        return control->GetValue();
    }
};

// App Code

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
    std::unordered_map<const char *, std::unique_ptr<FieldWrapper>> idMap;
    wxButton *submit;
private:
    void AddComponent(wxPanel *parent, wxBoxSizer *sizer, FieldMetadata meta);
    void OnSubmit(wxCommandEvent& event);
    void OnEscape(wxKeyEvent& event);
};
enum
{
    ID_Submit = 20000
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

    submit = new wxButton(panel, ID_Submit, "Submit");
    vbox->Add(submit, 1, wxEXPAND | wxALL, PADDING);

    Bind(wxEVT_BUTTON, &FormFrame::OnSubmit, this, ID_Submit);
    Bind(wxEVT_CHAR_HOOK, &FormFrame::OnEscape, this, wxID_ANY);
    // TODO: register ESC click handler: https://forums.wxwidgets.org/viewtopic.php?t=41926

    this->SetClientSize(panel->GetBestSize());
    this->CentreOnScreen();
}

void FormFrame::AddComponent(wxPanel *parent, wxBoxSizer *sizer, FieldMetadata meta) {
    void * control = nullptr;

    switch (meta.fieldType) {
        case FieldType::LABEL:
        {
            const LabelMetadata *labelMeta = static_cast<const LabelMetadata*>(meta.specific);
            auto label = new wxStaticText(parent, wxID_ANY, labelMeta->text, wxDefaultPosition, wxDefaultSize);
            control = label;
            fields.push_back(label);
            break;
        }
        case FieldType::TEXT:
        {
            const TextMetadata *textMeta = static_cast<const TextMetadata*>(meta.specific);
            auto textControl = new wxTextCtrl(parent, NewControlId());
            textControl->ChangeValue(textMeta->defaultText);
            
            // Create the field wrapper
            std::unique_ptr<FieldWrapper> field((FieldWrapper*) new TextFieldWrapper(textControl));
            idMap[meta.id] = std::move(field);
            control = textControl;
            fields.push_back(textControl);
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
        default:
            // TODO: handle unknown field type
            break;
    }

    if (control) {
        sizer->Add((wxWindow*) control, 0, wxEXPAND | wxALL, PADDING);
    }
}

void FormFrame::OnSubmit(wxCommandEvent &event) {
    for (auto& field: idMap) {
        FieldWrapper * fieldWrapper = (FieldWrapper*) field.second.get();
        wxString value {fieldWrapper->getValue()};
        wxCharBuffer buffer {value.ToUTF8()};
        char * id = strdup(field.first);
        char * c_value = strdup(buffer.data());
        ValuePair valuePair = {
            id,
            c_value,
        };
        values.push_back(valuePair);
    }

    Close(true);
}

void FormFrame::OnEscape(wxKeyEvent& event) {
    if (event.GetKeyCode() == WXK_ESCAPE) {
        Close(true);
    }else{
        event.Skip();
    }
}

extern "C" void interop_show_form(FormMetadata * _metadata, void (*callback)(ValuePair *values, int size, void *data), void *data) {
    SetProcessDPIAware();
    metadata = _metadata;
    wxEntry(0, nullptr);
    callback(values.data(), values.size(), data);

    // Free up values
    for (auto pair: values) {
        free((void*) pair.id);
        free((void*) pair.value);
    }
}