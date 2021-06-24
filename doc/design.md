# Design

## マウス・カーソル位置の名前

![20210622blog7a2.png](./img/20210622blog7a2.png)  

* `A` - Pressed point
* `B` - Previous point
* `C` - Moved vector

## 画像上のピクセル位置の名前

![20210622blog7a3.png](./img/20210622blog7a3.png)  

* `A` - Previous cell
* `B` - End cell

## 距離の名前

![20210622blog8a1.png](./img/20210622blog8a1.png)  

* `A` - DX
* `B` - DY

## 微小な移動

![20210622blog6.png](./img/20210622blog6.png)  

👆　微小な移動なら、描画しません  

## 二点の補間

![20210624blog36.png](./img/20210624blog36.png)  

* `A` - Previous point
* `B` - End point

![20210624blog36a1.png](./img/20210624blog36a1.png)  

* `A` - Longer side, positive
* `B` - Shorter side, positive

A:B = 6:5  

![20210624blog36a2b1.png](./img/20210624blog36a2b1.png)  

![20210624blog36a3b2.png](./img/20210624blog36a3b2.png)  
