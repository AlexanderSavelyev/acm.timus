/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

import java.io.*;
import org.junit.Test;

/**
 *
 * @author Aleksandr_Savelev
 */
public class TaskTest {

   public TaskTest() {
   }

   @Test
   public void testRun1() throws Exception {
      //Reader reader = new FileReader("input.txt");
      String test = "4\n" +
"WBWB\n" +
"BWWW\n" +
"WWBW\n" +
"WBWB";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
      Writer writer = new OutputStreamWriter(System.out);

      new Task1458().run(reader, writer);
   }

}
