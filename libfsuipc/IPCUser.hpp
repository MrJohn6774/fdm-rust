#ifndef IPCUSER_H
#define IPCUSER_H

#include <IntSafe.h>
#include <mutex>
#include <vector>

#ifndef _WINDEF_
typedef WORD ATOM;
typedef void *PVOID;
typedef PVOID HANDLE;
typedef HANDLE HWND;
#endif

namespace FSUIPC {

enum class Error : int {
  OK = 0,
  OPEN = 1,
  NOFS = 2,
  REGMSG = 3,
  ATOM = 4,
  MAP = 5,
  VIEW = 6,
  VERSION = 7,
  WRONGFS = 8,
  NOTOPEN = 9,
  NODATA = 10,
  TIMEOUT = 11,
  SENDMSG = 12,
  DATA = 13,
  RUNNING = 14,
  SIZE = 15,
  NOPERMISSION = 16  // Operation not permitted DWORD error code 0x5
};

enum class Simulator : int {
  ANY = 0,
  FS98 = 1,
  FS2K = 2,
  CFS2 = 3,
  CFS1 = 4,
  FLY = 5,
  FS2K2 = 6,
  FS2K4 = 7,
  FSX = 8,
  ESP = 9,
  P3D = 10,
  FSX64 = 11,
  P3D64 = 12,
  MSFS = 13,
};

class IPCUser {
 public:
  ~IPCUser() { this->Close(); }

  bool Open(Simulator requestedVersion, Error* result);
  void Close();
  bool Write(DWORD offset, DWORD size, void* src, Error* result);
  bool Process(Error* result);

  bool Read(DWORD offset, DWORD size, void* dest, Error* result) {
    return this->ReadCommon(false, offset, size, dest, result);
  }
  bool ReadSpecial(DWORD offset, DWORD size, void* dest, Error* result) {
    return this->ReadCommon(true, offset, size, dest, result);
  }

 protected:
  DWORD Version;
  DWORD FSVersion;
  DWORD LibVersion = 2002;

  HWND windowHandle;  // FS6 window handle
  UINT msgId;         // Id of registered window message
  ATOM atom;          // Atom containing name of file-mapping object
  HANDLE mapHandle;   // Handle of file-mapping object
  BYTE* viewPointer;  // Pointer to view of file-mapping object
  BYTE* nextPointer;

  std::vector<void*> destinations;

 public:
  bool ReadCommon(bool special,
                  DWORD offset,
                  DWORD size,
                  void* dest,
                  Error* result);
};

}  // namespace FSUIPC

#endif