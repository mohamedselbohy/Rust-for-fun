fn main() {
    calling_owned();
    let a = "a"; let b = "b";
    longest_malfunctioning(a,b);
    calling_shortest_lifetime();
    calling_x_or_none(); // empty string
    
    ///// This comment block would make you lose your mind probably for no reason. Just ignore it if you may
    ///// Seriously finish the lesson first before troubling your mind with this nonsense.
    // let mut a = [1, 2, 3, 4];
    // a[0];
    // let _aa = &mut a[1]; // This has nothing to do with lifetimes but it's an interesting story I had to look into
    // let bb = &mut a[2]; // Here if you have a vector<&mut i32> you will need the vector to be mutable to perform *b[4] = 8;
    // let b = vec![bb]; // While if it was an array you can simply *b[4] = 8; without making it mutable
    // *b[4] = 7; // The reason behind that foolery is so deep. the Key secret here lies in the deref operator (*) 
    //            // In the array implementation of * you will find that you just cast the array into a slice which is an easy peasy task and you don't have to make the array mutable in order to convert it to a slice
    //            // While for the vector you will need to convert it to a mutable pointer with unsafe rust to modify that pointer and track to the location you want in the heap.
    //            // You may be smart and ask me but bro. the * is for the b[0] not for b itself. I would tell you that you're right, you're smart I'd give you that.
    //            // But I'm not talking about that * in my code. because here's the plot twist: b[0] in the vector need to be turned into a slice to handle mut indexing and that function guess what. needs to *self
    //            // however you might ask me too but brooo. Why mut_index when we are using an immutable vector or an array. It's because it's on the LHS of the assignment.
    //            // But BROOO. How is the array passing that if it's immutable. Because it is not required to be mutable to handle mut_index but it is if you would want to assign anything to the outcome of []
    //            // while the vector requires it to be mutable to handle mut_indexing to let you be able to use it on L.H.S. Fine let's explicitly use index() on the LHS to not encounter this issue.
    //            // bam here's the problem you can't. because [] = *.index() so? when was the last time you could dereference an & &mut i and put something into it?
    //            // never. right. You might think but hmmm why not use ** then in this case. because you can't perform the other * now. why is that? uncomment and ask the compiler.
    //            // yeah rust is SO DANG FUN! :) And there is no workaround at all you will have to succumb to the compilers orders and never complain! Or it may skin you alive.
    // println!("{:?}", a);
}
// this compiled without a problem. Why do you think is that? // because there is no ambiguity about the lifetime of the returned value
// this might be a little vague but it would be clear when we call the function I promise
fn longest_owned(x: &str, y: &str) -> String {
    if x.len() > y.len(){
        x.to_string()
    } else {
        y.to_string()
    }
}


fn calling_owned() {
    let a = "abc";
    // let _refb;
    let c;
    {
        let b = "bcdahaha";
        c = longest_owned(a, b); // READ THIS FIRST THEN THE IMPORTANT NOTICE BEFORE THE NEXT SCOPE

        // _refb = &b; //--> this would have been an error. Because b does not live as long as b if refb was used after the scope
        // this means that b ends in this scope but c still has access to its value because its value was copied to c.
        // but since c lives long enough then the lifetime of the returned value is now living long enough as c
        

        // HAVE YOU READ THE IMPORTANT NOTICE?

        // {
        //     let _d = c; //--> here the longest_owned return value's ownership is transferred to _d and so the lifetime of the returned value
        // }    //ends on this exact line I'm leaving the comment on if you uncomment the scope
        // Uncomment the scope to view the error.
    }
    println!("{}",c); 
}

// HERE HERE
// important notice:: In rust of course values have lifetimes but they are always determined by the lifetimes of the variables
// therefore we always think about lifetimes of variables instead
// remember we have 2 types of variables: 1- owned 2- references
// if a variable owns a value the value's lifetime is bound by the life time of its owner (the scope of declaration of the owner (either the original or the one after a move))
// for references remember that a value has only one owner and definitely the reference is never the owner therefore if you are dealing with references
// you have to give an information about the owner to identify how the return should be managed if it was also a reference
// if the return was not a reference and it was an owned value then hurray to you, you should never get your hands dirty with rust lifetimes.
// if it was then you have to know about the lifetime of all the references and who outlives who to let the result be the one with chosen lifetime.




// even if you clone x and y before returning them you still will encounter the same compilation error.
// here &str is concerned about the lifetime of the returned value since it's a reference and not an owned return
// if it was owned it would have the lifetime of the variable it was assigned to
// since it's a reference it would require an information about the lifetime of the returned reference
fn longest_malfunctioning(_x: &str, _y: &str) -> &'static str{ // I will explain what 'static means later after we understand annotaions
    "hello" // this now works because "hello" is static and it was determined during compilation so it outlives everything practically
    // if x.len() > y.len(){
    //     x
    // } else {
    //     y
    // } //For this now the result is either going to mimic the reference of x or y and for that the result needs to know the lifetime
    // of the owner of x and y to know what will its lifeteime be. In this case we want to tell the compiler that the return should have the shortest lifetime of the parameters.
    // Even if the return was going to be the one to live longer but we just want to be safe and not let deterministic approaches ruin the safety of our references and cause dangling ones.
    // Even if you choose it to be the longer one. The compiler will still tell you that you can't return the other one in any case then
}


// let's now explore the lifetime annotation syntax
// just like when we say that <T> and specify a parameter x: &T. We are saying that T is going to be the type of x in order to restrict
// the other parameters or the return type if it was T as well.
// it's like &'a mut i32 and yeah it's only for references as we said.
// this does not restrict the call to let both the parameters have exactly the same lifetime. It just bounds the lifetime to the shortest of them.
fn longest_shortest_lifetime<'s>(x: &'s str, y: &'s str) -> &'s str{
    if x.len() > y.len() {
        x
    } else {
        y // if you remove 's from beside y in the parameter list you'd get an error check it out! then check out x_or_none below
    }
}

fn calling_shortest_lifetime() {
    let a = "ana".to_string();
    let b: String; // remove this to understand // Why does this work. Remember the value is bound to the owner's scope?
    let c;
    {
        b = "akjna".to_string(); // add "let" to understand
        c = longest_shortest_lifetime(a.as_str(), b.as_str());  //Don't let yourself be tricked if you removed to_string and as_str the error would be removed.
                                                                    // You might go crazy not understanding why the hell does it compile now while b definitely has a shorter lifetime than c
                                                                    // Well let's not be foolish you are using direct literals remember? They persist literally throughout the whole program and they outlive everyone
                                                                    // That's why it would work. And that's why I had to go through the trouble to put to_string and as_str to show you.
    }
    println!("{c}");
}

// watch this one to understand the difference

fn longest_x_or_none<'s>(x: &'s str, y: &str) -> &'s str {
    if x.len() > y.len(){
        x
    } else {
        ""
    }
}

fn calling_x_or_none(){
    let a = "kjsdnfk".to_string();
    let c;
    {
        let b = "kjsniufusbfykjs".to_string(); // here the owner's scope of b ends before c was used. C outlives b.
        c = longest_x_or_none(a.as_str(), b.as_str());
    }
    println!("{c}"); // c can be used while b can't. Yeah because the return type had nothing to do with b.
}

// Now you understand when to annotate and when to not.

// if a function only has one parameter the compiler infers that it should have the same lifetime.

// compiles because
fn same_single_param(x: &str) -> &str {
    let bytes = x.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &x[0..i];
        }
    }
    &x[..]
}

fn diff_single_param(x: &str) -> &'static str {
    if x.len() > 2{
        "afdss"
    } else{
        ""
    }
}

fn calling_diff(){
    let c;
    {
        let a = "a".to_string();
        c = diff_single_param(a.as_str()); // eventhough the return does not depend on a but it still whines about it not living long enough when removing 'static annotation
    }
    println!("{c}");
}

//'static means that it outlives everything

// congratulations we have covered half the chapters of the book! You are now officially a semi-rustacean