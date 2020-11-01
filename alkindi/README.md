# Alkindi

Letter frequency analysis using [Chi-squared test](https://en.wikipedia.org/wiki/Chi-squared_test):


Usage:

```rust
use alkindi;

fn main() {    
    let mut sums = vec![];
    let keys =  vec![
	"Vzz~|{r5XV2f5y|~p5t5ez`{q5zs5wtvz{",
	"]qquwpy>S]9m>rwu{>\u{7f}>nqkpz>qx>|\u{7f}}qp",
	"L``dfah/BL(|/cfdj/n/\u{7f}`zak/`i/mnl`a",
	"Cooking MC\'s like a pound of bacon",
	"Jffb`gn)DJ.z)e`bl)h)yf|gm)fo)khjfg"
	];
    
    
    for key in keys.iter() {    
        sums.push((key, alkindi::chi_sqr(key)));
    }

    sums.sort_by_key(|k| k.1 as u32);

    println!("---------------- DECRYPTION SUGGESTIONS -----------------");
    let mut count = 1;
    for s in sums {
        println!("{}. {:?}", count, s);
        count = count + 1;
    }        
}

```
output:

```bash
---------------- DECRYPTION SUGGESTIONS -----------------
1. ("Cooking MC\'s like a pound of bacon", 19.84736)
2. ("L``dfah/BL(|/cfdj/n/\u{7f}`zak/`i/mnl`a", 61.682438)
3. ("Jffb`gn)DJ.z)e`bl)h)yf|gm)fo)khjfg", 170.70404)
4. ("]qquwpy>S]9m>rwu{>\u{7f}>nqkpz>qx>|\u{7f}}qp", 834.97205)
5. ("Vzz~|{r5XV2f5y|~p5t5ez`{q5zs5wtvz{", 1044.558)
```
