
import javax.swing.*;
import java.awt.*;
import java.awt.event.*;
import java.io.IOException;
import java.util.ArrayList;

public class Task {
   
   

   public static void main(String[] args) throws IOException {

      JFrame frame = new JFrame("Test");
      frame.setVisible(true);
      frame.setSize(600, 600);
      frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
      
      int num_columns = 6;
      GridLayout experimentLayout = new GridLayout(num_columns, num_columns);

        

      JPanel panel = new JPanel();
      frame.add(panel);

      panel.setLayout(experimentLayout);
      
      ArrayList < ArrayList <JButton>> buttons = new ArrayList<>();
      
      for (int i = 0; i < num_columns; i++) {
         ArrayList<JButton> row = new ArrayList<>();
         buttons.add(row);
         for (int j = 0; j < num_columns; j++) {
            JButton button = new JButton("(" + i + "," + j + ")");
            panel.add(button);
            row.add(button);
            button.setBackground(Color.white);
            button.addActionListener (new PushAction(i, j, buttons)); 
         }
         
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
