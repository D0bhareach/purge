/*FIX
This is example file.
Note no space before FIX. No formatting so if you forget to format after purge
file will have empty lines on the top.
 */
/// This doc comment. This is going to stay.
fn main(){
    // FIX comment that I would like to remove
    // Note no : after FIX 
    let name = "D0bhareach";
    // PURGE must  be on separate line
    let name = "Other name";
    let i = 100;
    // ENDPURGE this line is going to be removed too.
    /* TODO
    long bla-bla about nothing.
     */

    /* FIX
    again long bla-bla.
     */
    println!("Hello, {}", name);
}