#include<iostream>
#include<iomanip>
using namespace std;
int main()
{
    int A,B,C,D,N;
    cin>>A;
    cin>>B;
    cin>>C;
    cin>>D;
    N = (A * B - C * D);
    cout<<"DIFERENCA = "<<fixed<<setprecision(1)<<N<<endl;

    return 0;
}
