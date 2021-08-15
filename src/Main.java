import java.util.Scanner;

public class Main {

    public static void main(String[] args) {
        System.out.println("Your Turn!! Please Choose Rock Paper or Sciccors \n 1.Rock \n 2.Paper \n 3.Scissors");

        Scanner scanner = new Scanner(System.in);
        int input = scanner.nextInt();
        int output = Math.max(1, 3);
        int[] possibleInputs = {1,2,3};
        boolean valid = false;

        for(var x : possibleInputs) {
            if (x == input) {
                valid = true;
                break;
            }
        }
        if(!valid) {
            System.out.println("Not A Valid Option");
            System.exit(0);
        }

        String result;
        String winning = "You won the match!!";
        String imagineLOL = "You lost the match";

        if(input == 2 && output == 1 || input == 1 && output == 3 || input == 3 && output == 2 ) {
            result = winning;
        } else if(input == output) {
            result = "Thats a draw";
        }else {
            result = imagineLOL;
        }
        System.out.println("You Chose: " + getName(input) + "\nOpponent Chose: " + getName(output));
        System.out.println(result);
    }

    public static String getName(int input) {

        if(input == 1) {
            return "Rock";
        }else if(input == 2) {
            return "Paper";
        }else if(input == 3) {
            return "Scissors";
        }

        return null;
    }

}
