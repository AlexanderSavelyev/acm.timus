/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

import java.io.*;
import static org.junit.Assert.assertEquals;
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
//      Writer writer = new OutputStreamWriter(System.out);
      
      StringWriter writer = new StringWriter();

      new Task1458().run(reader, writer);
      
      assertEquals("2\n2 3\n3 1", writer.getBuffer().toString().trim());
   }
   
      @Test
   public void testRun2() throws Exception {
      Reader reader = new FileReader("../visual/test1.txt");
//      Writer writer = new OutputStreamWriter(System.out);
      
      StringWriter writer = new StringWriter();

      new Task1458().run(reader, writer);
      
      assertEquals("4994\n" +
"1 2\n" +
"1 5\n" +
"1 7\n" +
"1 15\n" +
"1 16\n" +
"1 19\n" +
"1 21\n" +
"1 25\n" +
"1 26\n" +
"1 2", writer.getBuffer().toString().substring(0, 50));
   }
   
   @Test
   public void testRun3() throws Exception {
      Reader reader = new FileReader("../visual/test2.txt");
//      Writer writer = new OutputStreamWriter(System.out);
      
      StringWriter writer = new StringWriter();

      new Task1458().run(reader, writer);
      
      assertEquals("124709\n" +
"1 2\n" +
"1 5\n" +
"1 6\n" +
"1 7\n" +
"1 10\n" +
"1 16\n" +
"1 17\n" +
"1 19\n" +
"1 20\n" +
"1 ", writer.getBuffer().toString().substring(0, 50));
   }

}
