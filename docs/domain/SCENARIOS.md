# Scenarios

## Create Space
An **User** can create a **Space** to store his **Files** in **Directories**. He will be **Member** of this **Space** with **Own** **Permissions**. 

```
GIVEN User
  WHEN creates Space
  THEN CreateSpaceEvent
  WHEN space is created
  THEN SpaceCreatedEvent
```

## Delete Space
An **Member** with **Own** can delete a **Space**. If he does so, all **Files** and **Directories** are deleted and all **Members** removed.

```
GIVEN User WITH Own
  WHEN triggers space deletion
  THEN DeleteSpaceEvent
    THEN SpaceId invalid
    THEN delete all Directories and Files
  WHEN Space deleted
  THEN SeleteSpaceEvent
```

## Add Member to Space
A **Space** must have at least one **Member** and exactly one **Member** with **Own** **Permissions**. Such a **Member** can add a new **Member** by adding him via his **UserName**. A **Member** can *by default* **Download** all **Files** in the **Space**.

## Give and take permission from Member
A **Member** with **Own** **Permission** can give and take the **Permission** to **Upload** Files from **Members**.

## Transfer Own Permissions
A **Member** with **Own** **Permission** can transfer this **Permission** to another **Member**. It must only exist one **Member** with **Own**.

## Create Directory
A **Member** with **Upload** can create a **Directory**.

## Delete Directory
A **Member** with **Upload** can delete a **Directory**. It turns into a **DirectoryToDelete** until all of its files and nested **Directories** are removed. 

## Upload File
A **Member** with **Upload** can upload **Files** to a **Directory**. When the Upload is triggered, a **FileInUpload** is placed into the **Directory** as placeholder. When the upload is complete, it turns into a **StoredFile**.

## Delete File
A **Member** with **Upload** can delete a **StoredFile**. It turns into a **FileToDelete** until the deletion is complete and it is removed.
