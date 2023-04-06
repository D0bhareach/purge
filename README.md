### Agenda
CLI Application to remove special comment from source file.

### How it work
During development I use a lot of comments that are rather silly and only use to 
myself. For example TODO and  FIX comments I don't want to push these comments to
remote repository especially if it commit to public domain. So I need some tool
that prepare file to be pushed to repository.  
nocomment must take file as arg and remove unwanted comments. Plus sometime I have
test code or some setup (for example for logging) that is also no use in production.
I think to make app remove all code that between PURGE and END PURGE comments.