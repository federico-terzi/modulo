#ifndef MODULO_COMMON
#define MODULO_COMMON

#define _UNICODE

#include <wx/wxprec.h>
#ifndef WX_PRECOMP
    #include <wx/wx.h>
#endif

void setFrameIcon(const char * iconPath, wxFrame * frame);

void Activate(wxFrame * frame);

#endif