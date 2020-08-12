use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use plotlib::page::Page;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::style::BoxStyle;
use plotlib::view::ContinuousView;
//use std::fs::File;
////use std::io::{BufReader,Write};
//extern crate csv;


fn simulate_die_throw()->u64 {
    let mut rng = thread_rng();
    let dice = Uniform::new_inclusive(1, 6);
    
    
    let mut throws=0u64;
    let mut sixcounter=0;
    loop{
        let x=rng.sample(dice);
       // println!("After throwing the dice {} side came up", x);
        throws +=1;
        if x==6{
            sixcounter += 1;
    
        if sixcounter==2{
            return throws;
        }
        } else{
            sixcounter =0;
        }
    }
}


fn main(){
 let mut no_of_throws_until_doublesix: Vec<f64>= Vec::new();
 let mut  experiments=0;
    loop{
        let alpha=simulate_die_throw();
        no_of_throws_until_doublesix.push(alpha as f64);
        experiments += 1;

        if experiments==1000000{
            break;
        }
    }
  
  
    
    let sum: f64 = no_of_throws_until_doublesix.iter().sum();

    let mean = sum /no_of_throws_until_doublesix.len() as f64;
    
    println!("Throwing the dice {} times we get average no of throws to see a double six: {}",experiments,mean);

   
    let data = no_of_throws_until_doublesix;
   let h = Histogram::from_slice(&data, HistogramBins::Count(1))
       .style(&BoxStyle::new().fill("burlywood"));

   let v = ContinuousView::new().add(h);

    Page::single(&v).save("histogram_1.svg").expect("saving svg");
    
    let h = Histogram::from_slice(&data, HistogramBins::Count(10))
    .style(&BoxStyle::new().fill("burlywood"));

let v = ContinuousView::new().add(h);

 Page::single(&v).save("histogram_2.svg").expect("saving svg");

 let h = Histogram::from_slice(&data, HistogramBins::Count(100))
 .style(&BoxStyle::new().fill("burlywood"));

let v = ContinuousView::new().add(h);

Page::single(&v).save("histogram_3.svg").expect("saving svg");
}