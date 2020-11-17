import 'dart:io';
void main(){
  int num1,num2,num3,num4;

  num1 = int.parse(stdin.readLineSync());
  num2 = int.parse(stdin.readLineSync());
  num3 = int.parse(stdin.readLineSync());
  num4 = int.parse(stdin.readLineSync());

  int mul = (num1 * num2) - (num3 * num4);
  print("DIFERENCA = $mul");
}
