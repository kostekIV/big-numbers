use std::ops::{Add, Mul};
use std::fmt;

use crate::base_ops;
use crate::conversions::{convert_from_string, convert};


#[derive(Debug)]
pub struct Bint {
    base: u64,
    size: i32,
    repr: Vec<u64>,
}

impl Bint {
    fn zero(base: u64) -> Bint {
        Bint {
            base,
            size: 0,
            repr: Vec::new(),
        }
    }

    fn one(base: u64) -> Bint {
        Bint {
            base,
            size: 1,
            repr: Vec::from([1]),
        }
    }

    fn new(base: u64, value: u64, sign: bool) -> Bint {
        let mut sign = if sign {1} else {0};
        if value == 0 {
            sign = 0;
        }
        Bint {
            base,
            size: sign * 1,
            repr: Vec::from([value]),
        }
    }

    pub fn from_repr(base: u64, repr: Vec<u64>) -> Bint {
        Bint {
            base,
            size: repr.len() as i32,
            repr
        }
    }

    pub fn to_string() -> String {
        return "".to_string();
    }
}

impl <'a> Add for &'a Bint {
    type Output = Bint;

    fn add(self, other: &'a Bint) -> Bint {
        let repr = base_ops::add(&self.repr, &other.repr, self.base);

        Bint {
            base: self.base,
            size: repr.len() as i32,
            repr,
        }
    }
}

impl <'a> Mul for &'a Bint {
    type Output = Bint;

    fn mul(self, other: &'a Bint) -> Bint {
        let repr = base_ops::mul(&self.repr, &other.repr, self.base);

        Bint {
            base: self.base,
            size: repr.len() as i32,
            repr,
        }
    }
}

impl From<(u64, u64, &str)> for Bint {
    fn from(b_number: (u64, u64, &str)) -> Self {
        let (from, to, number) = b_number;
        let repr = convert_from_string(from, to, number.to_string());

        Bint::from_repr(to, repr)
    }
}

impl fmt::Display for Bint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut reversed_repr = self.repr.to_vec();
        reversed_repr.reverse();
        let dec = convert(self.base, 10, &reversed_repr);
        let str_repr: String = dec
                .into_iter()
                .rev()
                .map(|i| i.to_string())
                .collect::<String>();

        write!(f, "{}", str_repr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_work() {
        let base = u64::pow(2, 32);
        let a = Bint::new(base, 2000, true);
        let b = Bint::new(base, 4000, true);

        let mut res = &a + &b;

        for _ in 0..1000 {
            res = &res + &res;
            res = &res + &b;
        }
    }

    #[test]
    fn mul_work() {
        let base = u64::pow(2, 32);
        let a = Bint::new(base, 2000, true);
        let b = Bint::new(base, 4000, true);

        let mut res = &a * &b;

        for _ in 0..1000 {
            res = &(&res * &b) + &a;
        }
    }

    #[test]
    fn large_mul_work() {
        let base = u64::pow(2, 32);
        let a = Bint::from((10u64, base, "23984702938714092873409218734091287340981273"));
        let b = Bint::from((10u64, base, "21398470829374098127340821734"));

        let res = "513235966185276723577042802838626589685353980587603499037221157725387382";

        assert_eq!(res, (&a * &b).to_string());
    }

    #[test]
    fn factorial() {
        let base = u64::pow(2, 32);
        fn fact(base: u64, n: u64) -> Bint {
            if n == 0 {
                Bint::one(base)
            } else {
                &Bint::new(base, n, true) * &fact(base, n - 1)
            }
        }

        assert_eq!("1792233667382633521618843263044232513197622942259968207385215805123682159320161029848328112148883186161436034535802659466205111867109614573242316954383604389464524535467759401326264883566523043560811873179996072188155290081861628010250468430411854935707396605833540921031884571521279145124581094374547412403086564118143957940727734634769439112260383017302489106932716079961487372942529947238400000000000000000000000000000000000000000000000000000000",
                    fact(base, 231).to_string());

        assert_eq!("1220136825991110068701238785423046926253574342803192842192413588385845373153881997605496447502203281863013616477148203584163378722078177200480785205159329285477907571939330603772960859086270429174547882424912726344305670173270769461062802310452644218878789465754777149863494367781037644274033827365397471386477878495438489595537537990423241061271326984327745715546309977202781014561081188373709531016356324432987029563896628911658974769572087926928871281780070265174507768410719624390394322536422605234945850129918571501248706961568141625359056693423813008856249246891564126775654481886506593847951775360894005745238940335798476363944905313062323749066445048824665075946735862074637925184200459369692981022263971952597190945217823331756934581508552332820762820023402626907898342451712006207714640979456116127629145951237229913340169552363850942885592018727433795173014586357570828355780158735432768888680120399882384702151467605445407663535984174430480128938313896881639487469658817504506926365338175055478128640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
                    fact(base, 500).to_string());

        assert_eq!("771053011335386004144639397775028360595556401816010239163410994033970851827093069367090769795539033092647861224230677444659785152639745401480184653174909762504470638274259120173309701702610875092918816846985842150593623718603861642063078834117234098513725265045402523056575658860621238870412640219629971024686826624713383660963127048195572279707711688352620259869140994901287895747290410722496106151954257267396322405556727354786893725785838732404646243357335918597747405776328924775897564519583591354080898117023132762250714057271344110948164029940588827847780442314473200479525138318208302427727803133219305210952507605948994314345449325259594876385922128494560437296428386002940601874072732488897504223793518377180605441783116649708269946061380230531018291930510748665577803014523251797790388615033756544830374909440162270182952303329091720438210637097105616258387051884030288933650309756289188364568672104084185529365727646234588306683493594765274559497543759651733699820639731702116912963247441294200297800087061725868223880865243583365623482704395893652711840735418799773763054887588219943984673401051362280384187818611005035187862707840912942753454646054674870155072495767509778534059298038364204076299048072934501046255175378323008217670731649519955699084482330798811049166276249251326544312580289357812924825898217462848297648349400838815410152872456707653654424335818651136964880049831580548028614922852377435001511377656015730959254647171290930517340367287657007606177675483830521499707873449016844402390203746633086969747680671468541687265823637922007413849118593487710272883164905548707198762911703545119701275432473548172544699118836274377270607420652133092686282081777383674487881628800801928103015832821021286322120460874941697199487758769730544922012389694504960000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
                    fact(base, 800).to_string());
    }
}

