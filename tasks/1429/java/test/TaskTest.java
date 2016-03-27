/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

import java.io.*;
import java.util.LinkedList;
import static org.junit.Assert.assertEquals;
import org.junit.Test;

/**
 *
 * @author Aleksandr_Savelev
 * 26: 
 * r1 = 64
 * correct = + 4
 * 
 * 27:
 * correct = +1
 * r1 = 475
 * 
 * 30:
 * r1 = 140
 * correct = +0
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

   @Test
   public void testRun3() throws Exception {
      //Reader reader = new FileReader("input.txt");
      String test = "2\n"
              + "0 0 10000\n"
              + "9999 1 1";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
      StringWriter writer = new StringWriter();
      new Task().run(reader, writer);
      assertEquals("4", writer.getBuffer().toString().trim());
   }

   @Test
   public void testRun4() throws Exception {
      //Reader reader = new FileReader("input.txt");
      String test = "2\n"
              + "1 1 5\n"
              + "1 1 5";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
      StringWriter writer = new StringWriter();
      new Task().run(reader, writer);
      assertEquals("2", writer.getBuffer().toString().trim());
   }

   @Test
   public void testRun5() throws Exception {
      Reader reader = new FileReader("test/wa7.txt");
      StringWriter writer = new StringWriter();
      new Task().run(reader, writer);
      assertEquals("95847", writer.getBuffer().toString().trim());
   }

   @Test
   public void testRun6() throws Exception {
      //Reader reader = new FileReader("input.txt");
      String test = "2\n"
              + "0 0 2\n"
              + "0 0 3";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
      StringWriter writer = new StringWriter();
      new Task().run(reader, writer);
      assertEquals("3", writer.getBuffer().toString().trim());
   }

   @Test
   public void testVertices() {
      Task t = new Task();
      Task.Circle c1 = t.new Circle(0, 0, 3);
      Task.Circle c2 = t.new Circle(4, 0, 2);
      LinkedList<Task.Pair<Double, Double>> p = c1.calculateVertex(c2);
      assertEquals(2, p.size());

      c1 = t.new Circle(-5, -5, 3);
      c2 = t.new Circle(-1, -5, 2);
      p = c1.calculateVertex(c2);
      assertEquals(2, p.size());

      c1 = t.new Circle(0, 0, 3);
      c2 = t.new Circle(4, 3, 2);
      p = c1.calculateVertex(c2);
      assertEquals(1, p.size());
      Task.Pair<Double, Double> p1 = p.get(0);
      p = c2.calculateVertex(c1);
      Task.Pair<Double, Double> p2 = p.get(0);
      assertEquals(p1, p2);

//      c1 = t.new Circle(0, 0, 3);
//      c2 = t.new Circle(0, 0, 2);
//      p = c1.calculateVertex(c2);
//      assertEquals(1, p.size());
   }

   @Test
   public void testRun7() throws Exception {
      //Reader reader = new FileReader("input.txt");
      String test = "2\n"
              + "0 0 10000\n"
              + "9999 -1 1";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
      StringWriter writer = new StringWriter();

      
      
      new Task().run(reader, writer);
      assertEquals("4", writer.getBuffer().toString().trim());

      test = "2\n"
              + "0 0 10000\n"
              + "-9999 1 1";
      reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
      
      
                                                                            writer = new StringWriter();
      new Task().run(reader, writer);
      assertEquals("4", writer.getBuffer().toString().trim());
      
      test = "2\n"
              + "0 0 10000\n"
              + "-9999 1000 1";
      reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
      writer = new StringWriter();
      new Task().run(reader, writer);
      assertEquals("3", writer.getBuffer().toString().trim());
   }

   @Test
   public void testRun8() throws Exception {
      String test = "3\n"
              + "0 0 3\n"
              + "0 -4 5\n"
              + "0 4 5";
      Reader reader = new InputStreamReader(new ByteArrayInputStream(test.getBytes()));
      StringWriter writer = new StringWriter();
      new Task().run(reader, writer);
      assertEquals("6", writer.getBuffer().toString().trim());
   }
}
