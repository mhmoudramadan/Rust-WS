/**
 *     Compound types can group multiple values into one type
 *      two primitive compound types: tuples and arrays.
 *      Arrays like [1, 2, 3]
 *      Tuples like (1, true)
 * 
 * 
 * 
 *   ******************** tuple **********
 * 
 *  1-is a collection of values of different types.
 *  Tuples are constructed using parentheses (), and each tuple itself is a value with type signature (T1, T2, ...),
 *  where T1, T2 are the types of its members.
 * 
 * 2- Functions can use tuples to return multiple values, as tuples can hold any number of values.
//  
// tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
// Tuples have a fixed length: once declared, they cannot grow or shrink in size.
// We create a tuple by writing a comma-separated list of values inside parentheses. 
//Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same
*/
/**
 * 
 *          The Array Type
 *  Another way to have a collection of multiple values is with an array. 
 *  Unlike a tuple, every element of an array must have the same type. 
 *  Unlike arrays in some other languages, arrays in Rust have a fixed length.
 *  
 *  We write the values in an array as a comma-separated list inside square brackets:
 */


fn main(){

    //  Ways To create Array

    let arr_1 = [1,2,3,4,5];
    
    let arr_2:[i8;5] = [15,40,50,59,50]; // specify [data type ;size]


    // [initial value for all elements ; no of elments]
    // Both are the same 
    let arr_3 = [3;5];  
    let arr_4 = [3,3,3,3,3];


    /**
     *  Access array element
     */
    let mut index = arr_1[0];
    println!("index is {}",index);
    index = arr_2[3];
    println!(" index is {}",index);
    
    println!("Array is {:?}",arr_1); // :? for whole array 
    println!("Array at index 1 is {}",arr_1[1]);


    /***
     * 
     * 
     *  tuple 
     * 
     */

    //  Ways To create tuple 


    let tuple_1 =(250u8,100i8,500u32,true ,'a',"Mahmoud"); 

    let tub_2:(i8,i32,u16,&str,char,bool) = (8,500,600,"mahmoud",'A',true);


    println!("Tuple is {:?}",tuple_1);
    // Output 
    // Tuple is (250, 100, 500, true, 'a', "Mahmoud")

    println!("Tuple is {:#?}",tuple_1);
    // Output 
    // Tuple is (
    //     250,
    //     100,
    //     500,
    //     true,
    //     'a',
    //     "Mahmoud",
    // )

    println!("Tuple is {:?}",tub_2);



}