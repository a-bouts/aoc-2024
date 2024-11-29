#[allow(dead_code)]
pub trait PrimeExt {
    fn is_prime(&self) -> bool;

    fn divisors(&self) -> Vec<Self> where Self: Sized;
}

impl PrimeExt for u64 {
    fn is_prime(&self) -> bool {
        for i in 2..*self {
            if i.is_prime() && self % i == 0 {
                return false
            }
        }

        true
    }

    fn divisors(&self) -> Vec<Self> where Self: Sized {

        let mut primes = vec![2];
        for i in 3..*self+1 {
            let mut is_prime = true;
            for p in &primes {
                if i%p == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                primes.push(i);
            }
        }

        if primes.contains(self) {
            return vec![*self];
        }

        let mut divisors = vec!();
        for p in primes {
            if self % p == 0 {
                divisors.push(p);
            }
        }
        divisors
    }
}

#[allow(dead_code)]
pub trait PpcmExt {
    fn ppcm(&self) -> u64;
}

impl PpcmExt for Vec<u64> {
    fn ppcm(&self) -> u64 {
        let mut ppcm = 1;

        for nbr in self {
            let divs = nbr.divisors();

            for divisor in divs {
                if ppcm%divisor != 0 {
                    ppcm = ppcm * divisor;
                }
            }
        }

        ppcm
    }
}

#[cfg(test)]
mod tests {
    use crate::tools::ppcm::PrimeExt;


    #[test]
    fn test_divisors() {
        let divisors = 4051u64.divisors();

        assert_eq!(4051u64, divisors[0]);
    }


}
