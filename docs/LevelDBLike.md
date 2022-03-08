[forked leveldb repo](https://github.com/zTgx/leveldb)  

### Features
* Keys and values are arbitrary byte arrays.
* Data is stored sorted by key.
* Callers can provide a custom comparison function to override the sort order.
* The basic operations are Put(key,value), Get(key), Delete(key).
* Multiple changes can be made in one atomic batch.
* Users can create a transient snapshot to get a consistent view of data.
* Forward and backward iteration is supported over the data.
* Data is automatically compressed using the Snappy compression library.
* External activity (file system operations etc.) is relayed through a virtual interface so users can customize the operating system interactions.

---

### Limits
* This is not a SQL database.
* Only a single process (possibly multi-threaded) can access a particular database at a time.
* There is no client-server support builtin to the library. An application that needs such support will have to wrap their own server around the library.

---

### Contents

Guide to header files:

* include/leveldb/db.h: Main interface to the DB: Start here.

* include/leveldb/options.h: Control over the behavior of an entire database, and also control over the behavior of individual reads and writes.

* include/leveldb/comparator.h: Abstraction for user-specified comparison function. If you want just bytewise comparison of keys, you can use the default comparator, but clients can write their own comparator implementations if they want custom ordering (e.g. to handle different character encodings, etc.).

* include/leveldb/iterator.h: Interface for iterating over data. You can get an iterator from a DB object.

* include/leveldb/write_batch.h: Interface for atomically applying multiple updates to a database.

* include/leveldb/slice.h: A simple module for maintaining a pointer and a length into some other byte array.

* include/leveldb/status.h: Status is returned from many of the public interfaces and is used to report success and various kinds of errors.

* include/leveldb/env.h: Abstraction of the OS environment. A posix implementation of this interface is in util/env_posix.cc.

* include/leveldb/table.h, include/leveldb/table_builder.h: Lower-level modules that most clients probably won't use directly.
