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
      String test = "3\n"
              + "-1 0 2\n"
              + "1 0 2\n"
              + "0 0 3";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
      //Writer writer = new OutputStreamWriter(System.out);
      StringWriter writer = new StringWriter();
      new Task().run(reader, writer);
      assertEquals("6", writer.getBuffer().toString().trim());
   }
   @Test
   public void testRun2() throws Exception {
      //Reader reader = new FileReader("input.txt");
      String test = "4\n"
              + "-1 0 2\n"
              + "1 0 2\n"
              + "0 0 3\n"
              + "10 10 1";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
      StringWriter writer = new StringWriter();
      new Task().run(reader, writer);
      assertEquals("7", writer.getBuffer().toString().trim());
   }

}
