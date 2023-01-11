use std::ffi::c_void;
use std::mem;
use windows::{Win32::{System::
    {Memory::
        {VirtualAllocEx, MEM_COMMIT, PAGE_EXECUTE_READWRITE},
     Diagnostics::Debug::WriteProcessMemory, LibraryLoader::{GetModuleHandleA, GetProcAddress}, Threading::{CreateRemoteThread, OpenProcess, PROCESS_ALL_ACCESS}},
     Foundation::HANDLE},
     s};


pub fn inject_dll(handle:HANDLE,path:&str){
    unsafe{
        let baseaddress=VirtualAllocEx(handle,
            None,
            path.len(),
            MEM_COMMIT, // MEMCOMMIT= 0X1000
            PAGE_EXECUTE_READWRITE); //READWRITE_EXECUTE= 0X40

            
    
        let write_memory= WriteProcessMemory(handle, 
            baseaddress,
            path.as_ptr() as *const c_void, 
            path.len(), 
            None);
        
        if !write_memory.as_bool(){
            panic!("DLL Injection Failed");
        }

        let dllhandle=GetModuleHandleA(s!("kernel32.dll"));
        let load_lib_w_func_addy=GetProcAddress(dllhandle.unwrap(),s!("LoadLibraryA") );
        
        // println!("{:?} ",dllhandle.unwrap() );

        let start_address_option =  Some(mem::transmute(load_lib_w_func_addy)) ;

        CreateRemoteThread(handle,
            None, 0,
            start_address_option,
            Some(baseaddress), 0, None).unwrap();
        }
}

pub fn main(){
    let pid :u32= 100; // Specify process' PID to inject into 
    let path:&str=r#"C:\D\E\yourmom.dll"#; // Specify dll to inject
    
    let proc_handle= unsafe{
        match OpenProcess(PROCESS_ALL_ACCESS, // PROCESS_ALL_ACCESS = 0x001FFFFF
            false, pid){
                Ok(val) => val,
                Err(_) => panic!("Cannot find a handle for the given process' pid")
            }
    };
    inject_dll(proc_handle, path)
}
