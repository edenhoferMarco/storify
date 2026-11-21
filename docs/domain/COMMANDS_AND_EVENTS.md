# Commands and Events
> Style: *Command* --> **Event**

## User
- *CreateUser* --> **UserCreated**
- *DeleteUser* --> **UserDeleted**

## Space
- *CreateSpace* --> **SpaceCreated**
- *DeleteSpace* --> **SpaceDeleted**
- *TransferOwnership* --> **OwnershipTransferred**

## Member
- *AddMember* --> **MemberAdded**
- *RemoveMember* --> **MemberRemoved**

## Permission
- *ChangePermission* --> **PermissionChanged**

## Directory
- *CreateDirectory* --> **DirectoryCreated**
- *DeleteDirectory* --> **DirectoryDeletionStarted**
- (internal) *CompleteDirectoryDeletion* --> **DirectoryDeleted**

## File
- *StoreFile* --> **FileStorageStarted**
- (internal) *CompleteFileStorage* --> **FileStored**
- *DeleteFile* --> **FileDeletionStarted**
- (internal) *CompleteFileDeletion* --> **FileDeleted**
