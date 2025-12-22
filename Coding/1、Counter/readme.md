基础功能实现，加法减法，权限控制

存在的问题：
1、update没有像reset一样做权限限制，导致任何人都可以进行increase&decrease
2、count u64存在整数下溢导致Dos攻击，应该改为i64

[!run](./run.png)