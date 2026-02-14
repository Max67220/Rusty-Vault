pub fn check() -> bool {
    // TODO: RDTSC / QPC / GetTickCount
    false
}

if is_being_debugged() { exit(0); }

let start_time = unsafe { get_time_cycles() };
    let current_exe = env::current_exe().unwrap_or_else(|_| exit(0));
    let mut file = File::open(current_exe).unwrap_or_else(|_| exit(0));
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap_or_else(|_| exit(0));
    let delimiter_pos = buffer.windows(MAGIC_DELIMITER.len()).rposition(|w| w == MAGIC_DELIMITER);
    let end_time = unsafe { get_time_cycles() };


// 2. Time Lock

    let mut corrupted_mode = false;
//RDTSC
    bool IsDebugged(DWORD64 qwNativeElapsed)
    {
        ULARGE_INTEGER Start, End;
        __asm
        {
            xor  ecx, ecx
            rdtsc
            mov  Start.LowPart, eax
            mov  Start.HighPart, edx
        }
    // ... some work
        __asm
        {
            xor  ecx, ecx
            rdtsc
            mov  End.LowPart, eax
            mov  End.HighPart, edx
        }
        corrupted_mode = (End.QuadPart - Start.QuadPart) > qwNativeElapsed;
    }

    if (end_time > start_time) && (end_time - start_time) > MAX_CYCLES {
        corrupted_mode = true;
    }

//QueryPerformanceCounter()
    bool IsDebugged(DWORD64 qwNativeElapsed)
    {
        LARGE_INTEGER liStart, liEnd;
        QueryPerformanceCounter(&liStart);
        // ... some work
        QueryPerformanceCounter(&liEnd);
        corrupted_mode = (liEnd.QuadPart - liStart.QuadPart) > qwNativeElapsed;
    }

//GetTickCount()
    bool IsDebugged(DWORD dwNativeElapsed)
    {
        DWORD dwStart = GetTickCount();
        // ... some work
        corrupted_mode = (GetTickCount() - dwStart) > dwNativeElapsed;
    }

    
