//**********************************************************************
// Copyright Patrick Sweeney 2022
// Licensed under the MIT license.
// See file LICENSE for details.
//**********************************************************************

//use windows::
//{
//    core::*, Win32::Foundation::*, Win32::System::*, Win32::Media::*,
//};

#[allow(dead_code)]
#[allow(non_snake_case)]
mod Caustic
{
   mod Base
   {
       pub unsafe fn SystemStartup() -> ::windows::core::Result<()>
       {
	   windows::Win32::System::Com::CoInitializeEx(None, windows::Win32::System::Com::COINIT_MULTITHREADED)?;
	   windows::Win32::Media::MediaFoundation::MFStartup(windows::Win32::Media::MediaFoundation::MF_SDK_VERSION, windows::Win32::Media::MediaFoundation::MFSTARTUP_FULL)
       }
       
       pub unsafe fn SystemShutdown()
       {
	   windows::Win32::Media::MediaFoundation::MFShutdown();
	   windows::Win32::System::Com::CoUninitialize()
       }
   }
}
