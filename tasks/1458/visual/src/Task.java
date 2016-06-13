
import javax.swing.*;
import java.awt.*;
import java.awt.event.*;
import java.io.BufferedInputStream;
import java.io.BufferedReader;
import java.io.ByteArrayInputStream;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.BitSet;
import java.util.Random;
import java.util.StringTokenizer;
import java.util.function.IntConsumer;
import java.util.logging.Level;
import java.util.logging.Logger;

public class Task {
   
   

   public static void main(String[] args) throws IOException {
      
      createBigTest();
      
      String input = "4\n" +
"WBWB\n" +
"BWWW\n" +
"WWBW\n" +
"WBWB";
      BufferedReader in= new BufferedReader(new InputStreamReader(new ByteArrayInputStream(input.getBytes())));
      int num_columns = Integer.parseInt(in.readLine());
      
      ArrayList<BitSet> matrix = new ArrayList<>();
      for (int i = 0; i < num_columns; i++) {
         BitSet row = new BitSet(num_columns);
         String tokens = in.readLine();
         for (int j =0; j < tokens.length(); ++j) {
            if(tokens.charAt(j) == 'W') {
               row.set(j);
            }
         }
         matrix.add(row);
      }

      JFrame frame = new JFrame("Test");
      frame.setVisible(true);
      frame.setSize(600, 600);
      frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
      
//      int num_columns = 6;
      GridLayout experimentLayout = new GridLayout(num_columns, num_columns);

        

      JPanel panel = new JPanel();
      frame.add(panel);

      panel.setLayout(experimentLayout);
      
      ArrayList < ArrayList <JButton>> buttons = new ArrayList<>();
      Random rnd = new Random();
      for (int i = 0; i < num_columns; i++) {
         ArrayList<JButton> row = new ArrayList<>();
         buttons.add(row);
         for (int j = 0; j < num_columns; j++) {
            JButton button = new JButton("(" + (i+1) + "," + (j+1) + ")");
            panel.add(button);
            row.add(button);
            
            button.setBackground(Color.white);
            if(matrix.get(i).get(j)) {
               button.setBackground(Color.green);
            }
//            if(rnd.nextBoolean()) {
//               button.setBackground(Color.green);
//            }
            button.addActionListener (new PushAction(i, j, buttons)); 
         }
         
      }
   }

   private static void createBigTest()  {
      int N = 500;
      Random rnd = new Random();
      try (FileWriter writer = new FileWriter("test2.txt")) {
         writer.append(N + "\n");
         for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
               if (rnd.nextBoolean()) {
                  writer.append('W');
               } else {
                  writer.append('B');
               }
            }
            writer.append('\n');
         }
      } catch (IOException ex) {
         ex.printStackTrace();
      } 
   }

   private static class PushAction implements ActionListener {

      private final int i;
      private final int j;
      private final ArrayList < ArrayList <JButton>> buttons;

      private PushAction(int i, int j, ArrayList < ArrayList <JButton>> buttons) {
         this.i = i;
         this.j = j;
         this.buttons = buttons;
      }

      @Override
      public void actionPerformed(ActionEvent e) {
         ArrayList<JButton> row = buttons.get(i);
         for (JButton jButton : row) {
            if(jButton.getBackground() == Color.white)
               jButton.setBackground(Color.green);
            else
               jButton.setBackground(Color.white);
         }
         
         for (ArrayList<JButton> crow : buttons) {
            if(crow.equals(row))
               continue;
            JButton jButton = crow.get(j);
            if(jButton.getBackground() == Color.white)
               jButton.setBackground(Color.green);
            else
               jButton.setBackground(Color.white);
         }
      }
   }

}
