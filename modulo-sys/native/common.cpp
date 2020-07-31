#include "common.h"

void setFrameIcon(const char * iconPath, wxFrame * frame) {
    if (iconPath) {
        wxString iconPath(iconPath);
        wxBitmapType imgType = wxICON_DEFAULT_TYPE;

        #ifdef __WXMSW__
            imgType = wxBITMAP_TYPE_ICO;
        #endif

        wxIcon icon;
        icon.LoadFile(iconPath, imgType);
        if (icon.IsOk()) {
            frame->SetIcon(icon);
        }
    }
}