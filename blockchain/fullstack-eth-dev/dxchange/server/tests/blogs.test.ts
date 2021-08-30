import supertest from 'supertest';
import mongoose from 'mongoose';
import list_helper from '../utils/list_helper';
import { IBlog, Blog } from '../models/blog';
import test_helper from './test_helper';
import app from '../app';

const api = supertest(app);

beforeEach(async () => {
  await Blog.deleteMany({});

  for (let blog of test_helper.initialBlogs) {
    const newBlog = await new Blog(blog);
    await newBlog.save();
  }
});

afterAll(() => {
  mongoose.connection.close();
});

describe('Blogs', () => {
  test('blogs are returned as json', async () => {
    await api
      .get('/api/v1/blogs')
      .expect(200)
      .expect('Content-Type', /application\/json/);
  });

  test('all blogs are returned', async () => {
    const response = await api.get('/api/v1/blogs');
    expect(response.body.length).toBe(test_helper.initialBlogs.length);
  });

  test('unique identifier property of the blog posts is named id', async() => {
    const response = await api.get('/api/v1/blogs');
    response.body.forEach((blog: any) => {
      expect(blog.id).toBeDefined()
      expect(blog).toHaveProperty('id');
    });
  })
  

  test('dummy returns one', () => {
    const blogs = [] as IBlog[];
    expect(list_helper.dummy(blogs)).toBe(1);
  });
});
