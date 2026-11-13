#include <cmath>
#include <iostream>
using namespace std;

int main() {
    long long T;
    cin >> T;

    for (long long _t = 0; _t < T; _t++) {
        // A and B stock
        long long As, Bs;
        // exchange rates
        long long cA, cB;
        // what we wish to guarantee
        long fA;

        cin >> As >> Bs >> cA >> cB >> fA;

        // if we already have the required A stock
        if (As >= fA) {
            cout << 0 << "\n";
            continue;
        }

        // exchange rates
        double rateBA = static_cast<double>(cB) / static_cast<double>(cA);
        double rateAB = static_cast<double>(cA) / static_cast<double>(cB);

        // if we already have enough convertible B stock and some A stock for
        // total required A stock
        if (Bs * rateAB + As >= fA) {
            cout << 0 << "\n";
            continue;
        }
        // cout << "Trial " << _t << ": rateBA = " << rateBA
        //      << ", rateAB = " << rateAB << ".\n";

        // if B:A conversion is high, then it would be better for you to just
        // get additional A chips but since this is a "guarantee" problem and
        // not a "optimal" problem, we have to imagine getting all additional B
        // chips
        if (rateBA >= 1) {
            // how much B you would need to get to fA by converting B:A, worst
            // case
            // cout << "(fA - As) * rateBA = " << (fA - As) * ceil(rateBA) <<
            // "\n";
            long long ans = ceil((fA - As) * rateBA);

            // make sure to have the right multiple of cB (e.g. you need to have
            // 3 B's in a 3:2 to do anything)
            if (ans % cB != 0) ans += cB - (ans % cB);

            // remove what we already have
            ans -= Bs;

            // you can "tease" the growing B stock by having exactly 1 less A
            // stock than a possible multiple of A from B:A
            // if (fA - As % cA != cA - 1) { ans += cA - 1 - (fA - As % cA); }
            if ((fA - As) % cA != cA - 1) {
                // cout << "fA - As = " << fA - As << "\n"
                //      << "cA - 1 = " << cA - 1 << "\n";
                ans += cA - ((fA - As) % cA) - 1;
            }

            cout << ans << "\n";
        } else {
            // should this not be floor?
            cout << fA - floor(As - rateAB) * Bs << "\n";
        }
    }
}
