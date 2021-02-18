#include <iostream>
#include <fstream>
#include <gmpxx.h>

using namespace std;


int main() {
  fstream div_file, rem_file;
  div_file.open("div_test.csv", ios::out);
  rem_file.open("rem_test.csv", ios::out);

  mpz_class a, b;
  int n = 300;
  gmp_randclass ran(gmp_randinit_default);
  ran.seed(time(NULL));

  while (n--) {
      a = ran.get_z_bits(4000);
      b = ran.get_z_bits(2000);
      auto q = mpz_class(a / b);
      auto r = mpz_class(a % b);
      div_file << a.get_str() << "," << b.get_str() << "," << q.get_str() << "\n";
      rem_file << a.get_str() << "," << b.get_str() << "," << r.get_str() << "\n";
  }

  n = 100;
  while (n--) {
      a = ran.get_z_bits(10000);
      b = ran.get_z_bits(5000);
      auto q = mpz_class(a / b);
      auto r = mpz_class(a % b);
      div_file << a.get_str() << "," << b.get_str() << "," << q.get_str() << "\n";
      rem_file << a.get_str() << "," << b.get_str() << "," << r.get_str() << "\n";
  }  n = 100;

  while (n--) {
      a = ran.get_z_bits(10000);
      b = ran.get_z_bits(5000);
      auto q = mpz_class(a / b);
      auto r = mpz_class(a % b);
      div_file << a.get_str() << "," << b.get_str() << "," << q.get_str() << "\n";
      rem_file << a.get_str() << "," << b.get_str() << "," << r.get_str() << "\n";
  }

  n = 100;
  while (n--) {
      a = ran.get_z_bits(1000);
      b = ran.get_z_bits(10);
      auto q = mpz_class(a / b);
      auto r = mpz_class(a % b);
      div_file << a.get_str() << "," << b.get_str() << "," << q.get_str() << "\n";
      rem_file << a.get_str() << "," << b.get_str() << "," << r.get_str() << "\n";
  }

  div_file.close();
  rem_file.close();
}
