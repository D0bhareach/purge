### About
CLI Application to remove marked section from source file.    
This tool take file as arg and remove unwanted comments. During development I use a lot
of comments that are rather only use to myself. For example TODO and  FIX comments
I don't want to push these comments to remote repository especially if it commit to public
domain. So I need some tool that prepare file to be pushed to repository.    
Another usage is when I have test code or some setup (for example for logging) that is
also no use in production. For example I commit to fork and my setup shall not be in common
branch.   
I think to make app remove all code that between PURGE and ENDPURGE comments.
PURGE  & ENDPURGE comments must be on sepatate line from code. This lines are going to
be deleted as well.
I think it would be nice to have this tool to look for different comment styles
depending on file extension. Bash, Python, toml for example are commented with `#`,
while rs, java, ts are commented with slashes `/`.    
Because this tool is not properly tested and bugs are possible I leave automatic purging for
directories off for now. 

### What is done
<b>IMPORTANT! This tool rewriting files change them automatically. I plan to use this tool
as automation for leaning the code on long running branch dedicated for cleaning and last
minute adjustments.</b>   
Inline comments removal. `/*!!` is used to mark the start of inline comment that this tool
will remove. The marker is only works for comment on the same line. Tool will not search for
match through other lines.      
Lines started with // FIX or // TODO are marking block of comments that will be removed up
to empty line or line without `//` on the start. IMPORTANT! This tool will remove whole line
marked with start `// FIX` and end `//` so they must be on the line without anything that
shall not be touched. Use /*!! Bla-bla*/ instead.  
Line that have `//PURGE` is going to be deleted together with anyting below. Line // ENDPURGE
is going to be deleted and swith deletion of the others lines off.    
All markers must contain their ending characters! File is not formatted after lines are purged
for Rust code I use `rustfmt` after purge is complete.

### About development
I thought that will do it in one evening. Looks like it is not going to happen.I don't 
have plans to write full dev documentation for this tool. So I will collect some useful links
about varios topics here.   

#### ANSI Colors for prints.
I wouldn't bring special dependency just to make lines in stdout colored.
[colored lib](https://docs.rs/crate/colored/2.0.0) is a dependency. I would like to avoid. [Answer from Stackoverflow](https://stackoverflow.com/questions/69981449/how-do-i-print-colored-text-to-the-terminal-in-rust) gave me some idea and [this reference to codes](https://ss64.com/nt/syntax-ansi.html) is useful and sufficient to write whichever loggings I fancy.

#### How to work with nodes of code.
It's easy when you have synchronous walk on something and never have to change anything. At the moment I see
one possible walk the code. When searching for one line simple comments marked with markers and it never has 
to go back and forch.   
It was easier for me instead of using stream and seek back and forch, simply use 'by line approach'.   

#### TODO
I hard code capacity of vector holding lines to 250. Perhaps I should count lines first. I didn't account for
security concern of virtual memory overflow. At the end it's simple tool. I have no intention to shoot myself
in the foot.  
I'm too buzy and important (kidding) and too lazy (true) I haven't done other comments. I mean any file that
commented with `#` is not accounted for yet. Have plans, though. toml files are not going to be cleand by
themselfs.   
Tests not done yet. And I really should. Besides, test is far more productive way to procrastinate, then, 
for example, searching for a job.   
No wrapped commets are supported yet. Behaviour for such comments may be faliable.   