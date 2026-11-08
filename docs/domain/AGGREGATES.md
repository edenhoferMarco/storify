# Aggregates
> Topmost Entity is the Aggregate Root

## Core Domain: Filesystem
### StoredFile 
```mermaid
flowchart TB
    StoredFile --> FileName
    StoredFile --> DirectoryId
```
### Directory
```mermaid
flowchart TB
    Directory --> DirectoryName
    Directory --> DirectoryId
    Directory --> FileId
    Directory --> SpaceId
```
  
### Space
```mermaid
flowchart TB
    Space --> SpaceId
    Space --> SpaceName
    Space --> DirectoryId
    Space --> Member
    Member --> Permission
    Member --> UserId
```

## Sub Domain: Users
### User
```mermaid
flowchart TB
    User --> UserName
    User --> UserId
```

# Entities and Value Objects
## Entities
- StoredFile
- Directory
- Space
- Member
- User

## Value Objects
- FileName
- FileId
- DirectoryName
- DirectoryId
- SpaceName
- SpaceId
- UserId
- UserName
- Permission