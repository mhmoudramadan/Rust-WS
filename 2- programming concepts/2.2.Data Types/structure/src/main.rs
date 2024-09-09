
// There are three types of structures ("structs") that can be created using the struct keyword:
//     1-Tuple structs, which are, basically, named tuples.
//     2-The classic C structs
//     3-Unit structs, which are field-less, are useful for generics.


//  Ways to create Structs
  // Tuple structs
  struct Unit_(u16,bool);

  // classic C struct
 
  #[derive(Debug)]
  struct Person{
      name: String,
      age : u8,
      email: String,
  }


  //  unit struct 

  struct Unit;

  struct Dimension{
      x:f32,
      y:f32,
  }

  // reused as fields of another struct
  struct Rectangle {
      height:Dimension,
      width:Dimension,
  }


fn main() {

    //  Ways to create objects and Accessing struct elements
    let info = Person{
        name:String::from("Mahmoud"),
        age:28,
        email:String::from("mmmm@aaa.com"),
    };

    println!("Person info is {:?}",info);
    println!("name is {} age is {} email is {}",info.name,info.age,info.email);

    let name = String::from("Mahmoud");
    let age = 28;
    let email = String::from("aaa.aaa.com");

    let mahmoud = Person{name,age,email};

    println!("Person info is {:?}",mahmoud);


    // unit struct 
    // Access tuple struct done by index of data type of T

    let unit_= Unit_(100,true);

    println!("tuple struct is {},{}",unit_.0,unit_.1);

    // 
    let dim1 = Dimension{x:2.5,y:4.0};
    let dim2:Dimension = Dimension{x:2.0,y:5.0};

    println!("Dimension is {} {}",dim1.x,dim1.y);

    let rec1:Rectangle = Rectangle{
        height : dim1,
        width : dim2,
    };

    println!("Rectangle is {} ",rec1.height.x);
}
