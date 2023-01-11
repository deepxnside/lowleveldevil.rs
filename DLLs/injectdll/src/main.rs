use std::ffi::c_void;
use std::mem;
use windows::{Win32::{System::
    {Memory::
        {VirtualAllocEx, MEM_COMMIT, PAGE_EXECUTE_READWRITE},
     Diagnostics::Debug::WriteProcessMemory, LibraryLoader::GetModuleHandleA, Threading::{CreateRemoteThread, OpenProcess, PROCESS_ALL_ACCESS}},
     Foundation::HANDLE},
     s};


pub fn InjectDll(handle:HANDLE,path:&str){

    unsafe{
        let baseaddress=VirtualAllocEx(handle,
        None,
        path.len(),
        MEM_COMMIT, // MEMCOMMIT= 0X1000
        PAGE_EXECUTE_READWRITE); //READWRITE_EXECUTE= 0X40
    
            WriteProcessMemory(handle, 
                baseaddress,
                path.as_ptr() as *const c_void, 
                path.len(), 
                None);
    
            
            // GetModuleHandleA(s!("LoadLibraryA.dll"));

            let start_address_option = unsafe { Some(mem::transmute(baseaddress)) };
    

            CreateRemoteThread(handle,
                None,
                0,
                start_address_option,
                None,
                0,
                None);
            }
}


pub fn main(){
    let pid :u32= 100; // Specify process' PID to inject into 
    let path:&str=r#""#; // Specify dll to inject
    
    let proc_handle= unsafe{
        match OpenProcess(PROCESS_ALL_ACCESS, // PROCESS_ALL_ACCESS = 0x001FFFFF
            false, 
            pid){
                Ok(val) => val,
                Err(_) => panic!("Cannot find a handle for the given process' pid")
            }
    };

    InjectDll(proc_handle, path)
}