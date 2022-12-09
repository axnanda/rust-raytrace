use std::convert::TryInto;
use std::mem;
fn main() {
    use graplot::Plot3D;
    //add ability to change the bounds of the graph
    //you can't do 3 cos(&t&) because you have to explicetly identify the multiplicaitona dna division that takes place
    use std::io::{stdin,stdout,Write};
    let mut foft=String::new();
    print!("Please enter the x component of some vector-valued function in the form (x(&t&))  ex: (3cos(&t&)): ");
    let _=stdout().flush();
    stdin().read_line(&mut foft).expect("You did not enter a correct vector-valued function, try again");
    if let Some('\n')=foft.chars().next_back() {
        foft.pop();
    }
    if let Some('\r')=foft.chars().next_back() {
        foft.pop();
    }
    let mut goft=String::new();
    print!("Please enter the y component of some vector-valued function in the form (y(&t&))  ex: (5sin(&t&)): ");
    let _=stdout().flush();
    stdin().read_line(&mut goft).expect("You did not enter a correct vector-valued function, try again");
    if let Some('\n')=goft.chars().next_back() {
        
        goft.pop();
    }
    if let Some('\r')=goft.chars().next_back() {
        goft.pop();
    }
    let mut hoft=String::new();
    print!("Please enter the z component of some vector-valued function in the form (z(&t&))  ex: (2sin(&t&)): ");
    let _=stdout().flush();
    stdin().read_line(&mut hoft).expect("You did not enter a correct vector-valued function, try again");
    if let Some('\n')=hoft.chars().next_back() {
        hoft.pop();
    }
    if let Some('\r')=hoft.chars().next_back() {
        hoft.pop();
    }
    let mut stringEvals=String::new();
    print!("How many points would you like to include in your graph:");
    let _=stdout().flush();
    stdin().read_line(&mut hoft).expect("You did not enter a correct vector-valued function, try again");//is this needed
    if let Some('\n')=hoft.chars().next_back() {
        hoft.pop();
    }
    if let Some('\r')=hoft.chars().next_back() {
        hoft.pop();
    }
    //could you just choose a few traces such as all lines from y=x and rotate by some value and then use z values and draw circles

    let numEvals = meval::eval_str(stringEvals).unwrap();
    let numIter: i64 = numEvals as i64;
    let numUsize: usize = numEvals as usize;
    let mut xs = vec![0.; numUsize];
    let mut ys = vec![0.; numUsize];
    let mut zs = vec![0.; numUsize];
    
    
    //now need to add user input to specify the bounds of the graph
    //and need to add ability to set bounds between
    //lowest smoothing curve or just some way to generate splines between areas
    for n in 0..numIter {
        let mut m = n as i64;
        m = m - numIter/2;
        let mut newfoft=foft.clone();
        let mut newgoft=goft.clone();
        let mut newhoft=hoft.clone();
        newfoft = newfoft.replace("&t&", &m.to_string());
        newgoft = newgoft.replace("&t&", &m.to_string());
        newhoft = newhoft.replace("&t&", &m.to_string());
        let myx = meval::eval_str(newfoft).unwrap();
        let myy = meval::eval_str(newgoft).unwrap();
        let myz = meval::eval_str(newhoft).unwrap();
        xs[n as usize]=myx as f64;
        ys[n as usize]=myy as f64;
        zs[n as usize]=myz as f64;
        //why does cos or sin or tan not work
    }
    //then use the points to produce lowest smoothing curves between points
    //WHAT IS TYPE ANNOTATIONS NEEDED?
    let mut fts = vector_to_array(xs);
    let mut gts = vector_to_array(ys);
    let mut hts = vector_to_array(zs);
    let plot = Plot3D::new((fts, gts, hts, "r-o"));
    plot.show();
}

fn convert_vec_to_array<T: Copy>(vec: Vec<T>, size: usize) -> [T; size] {
    let mut array: [T; size] = unsafe { mem::uninitialized() };
    for (i, item) in vec.into_iter().enumerate() {
        array[i] = item;
    }
    array
}


fn vector_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn cast_vec<T, U>(vec: Vec<T>) -> Vec<U> {
    unsafe {
        let vec_ptr = vec.as_mut_ptr();
        let vec_len = vec.len();
        let vec_cap = vec.capacity();
        Vec::from_raw_parts(vec_ptr as *mut U, vec_len, vec_cap)
    }
}