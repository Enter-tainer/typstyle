$
U(P)
  &= - G integral_(r=0)^R integral_(theta = 0)^(pi) integral_(phi=0)^(2 pi) rho(r) / q r^2 dif r dif theta sin(
    theta,
  ) dif phi
  &"on choisit de prendre" phi in [0,2 pi[, theta in [0, pi[\
  &=- 2 pi G integral_(r=0)^R integral_(theta = 0)^(pi) rho(r) / q r^2 dif r dif theta sin(
    theta,
  ) &"intégration par rapport à" phi "indépendant de" rho\
  &=- 2 pi G integral_(r=0)^R integral_(theta = 0)^(pi) rho(r) / q r^2 dif r dif theta sin(
    theta,
  ) &"intégration par rapport à" phi "indépendant de" rho\
  &=- 2 pi G integral_(r=0)^R integral_(theta = 0)^(pi) rho(r) / q 11 r^2 dif r dif theta sin(
    theta,
  ) &"intégration par rapport à" phi "indépendant de" rho\
  &= - 2 pi G integral_(r=0)^R integral_(theta = 0)^(pi) rho(r) / (
    r^2 + s^2 - 2 r s cos(theta)
  )^(1\/2) r^2 dif r dif theta
  &"formule" q^2 = r^2 + s^2 - 2 r s cos(theta) quad (1)\
  &= - 2 pi G
  integral_(r=0)^R
  integral_(u=-1)^1
  (rho(r)) / ((r^2 + s^2 - 2 r s u)^(1\/2)) r^2 dif r dif u
  &u = cos(theta), dif u = - sin(theta) d theta\
  &=
  - 2 pi G integral_(r=0)^R rho(r) r^2 integral_(u=-1)^1 (dif u) / (((r^2 + s^2) + (-2 r s) u)^(1\/2)) dif r
  &integral (dif x) / sqrt(a + b x) = (2 sqrt(a + b x)) / b\
  &= - 2 pi G integral_(r=0)^R rho(r) r^2 [2 sqrt(r^2 + s^2 - 2 r s u) / (- 2 r s)]_(-1)^1 dif r\
  &= - 2 pi G integral_(r=0)^R rho(r) r^2 1 / (r s) (sqrt(r^2 + s^2 + 2 r s) - sqrt(r^2 + s^2 - 2 r s)) dif r\
  &= - (2 pi G) / s integral_(r=0)^R rho(r) r (sqrt((r+s)^2) - sqrt((r-s)^2)) dif r\
  &= - (2 pi G) / s
  integral_(r=0)^R rho(r) r ((r +s) - (s-r)) dif r
  & sqrt((r-s)^2) = abs(r-s) = s-r "car" r < s\
  &= - (2 pi G) / s integral_(r=0)^R rho(r) r (2 r) dif r\
  &= - (2 pi G) / s integral_(r=0)^R rho(r) 2 r^2 dif r\
  &= - G / s integral_(r=0)^R rho(r) 4 pi r^2 dif r\
  &= - (G M) / s
$
