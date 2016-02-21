#[macro_export]
macro_rules! swizzle {
    (x, $ident:ident ) => {
		impl<T> $ident<T> {
			pub fn x(self) -> T { self.0 }
		}
    };

    (y, $ident:ident ) => {
		impl<T> $ident<T> {
			pub fn y(self) -> T { self.1 }
		}
    };

    (z, $ident:ident ) => {
		impl<T> $ident<T> {
			pub fn z(self) -> T { self.2 }
		}
    };

    (w, $ident:ident ) => {
		impl<T> $ident<T> {
			pub fn w(self) -> T { self.3 }
		}
    };

	(2, $ident:ident ) => {
		impl<T: Copy> $ident<T> {
			pub fn xx(self) -> Vec2<T> { Vec2(self.0, self.0) }
			pub fn xy(self) -> Vec2<T> { Vec2(self.0, self.1) }
			pub fn yx(self) -> Vec2<T> { Vec2(self.1, self.0) }
			pub fn yy(self) -> Vec2<T> { Vec2(self.1, self.1) }
		}
    };

	(3, $ident:ident ) => {
		impl<T: Copy> $ident<T> {
			pub fn xxx(self) -> Vec3<T> { Vec3(self.0, self.0, self.0) }
			pub fn xxy(self) -> Vec3<T> { Vec3(self.0, self.0, self.1) }
			pub fn xxz(self) -> Vec3<T> { Vec3(self.0, self.0, self.2) }
			pub fn xyx(self) -> Vec3<T> { Vec3(self.0, self.1, self.0) }
			pub fn xyy(self) -> Vec3<T> { Vec3(self.0, self.1, self.1) }
			pub fn xyz(self) -> Vec3<T> { Vec3(self.0, self.1, self.2) }
			pub fn xzx(self) -> Vec3<T> { Vec3(self.0, self.2, self.0) }
			pub fn xzy(self) -> Vec3<T> { Vec3(self.0, self.2, self.1) }
			pub fn xzz(self) -> Vec3<T> { Vec3(self.0, self.2, self.2) }
			pub fn yxx(self) -> Vec3<T> { Vec3(self.1, self.0, self.0) }
			pub fn yxy(self) -> Vec3<T> { Vec3(self.1, self.0, self.1) }
			pub fn yxz(self) -> Vec3<T> { Vec3(self.1, self.0, self.2) }
			pub fn yyx(self) -> Vec3<T> { Vec3(self.1, self.1, self.0) }
			pub fn yyy(self) -> Vec3<T> { Vec3(self.1, self.1, self.1) }
			pub fn yyz(self) -> Vec3<T> { Vec3(self.1, self.1, self.2) }
			pub fn yzx(self) -> Vec3<T> { Vec3(self.1, self.2, self.0) }
			pub fn yzy(self) -> Vec3<T> { Vec3(self.1, self.2, self.1) }
			pub fn yzz(self) -> Vec3<T> { Vec3(self.1, self.2, self.2) }
			pub fn zxx(self) -> Vec3<T> { Vec3(self.2, self.0, self.0) }
			pub fn zxy(self) -> Vec3<T> { Vec3(self.2, self.0, self.1) }
			pub fn zxz(self) -> Vec3<T> { Vec3(self.2, self.0, self.2) }
			pub fn zyx(self) -> Vec3<T> { Vec3(self.2, self.1, self.0) }
			pub fn zyy(self) -> Vec3<T> { Vec3(self.2, self.1, self.1) }
			pub fn zyz(self) -> Vec3<T> { Vec3(self.2, self.1, self.2) }
			pub fn zzx(self) -> Vec3<T> { Vec3(self.2, self.2, self.0) }
			pub fn zzy(self) -> Vec3<T> { Vec3(self.2, self.2, self.1) }
			pub fn zzz(self) -> Vec3<T> { Vec3(self.2, self.2, self.2) }
		}
    };

	(4, $ident:ident ) => {
		impl<T: Copy> $ident<T> {
			pub fn xxxx(self) -> Vec4<T> { Vec4(self.0, self.0, self.0, self.0) }
			pub fn xxxy(self) -> Vec4<T> { Vec4(self.0, self.0, self.0, self.1) }
			pub fn xxxz(self) -> Vec4<T> { Vec4(self.0, self.0, self.0, self.2) }
			pub fn xxxw(self) -> Vec4<T> { Vec4(self.0, self.0, self.0, self.3) }
			pub fn xxyx(self) -> Vec4<T> { Vec4(self.0, self.0, self.1, self.0) }
			pub fn xxyy(self) -> Vec4<T> { Vec4(self.0, self.0, self.1, self.1) }
			pub fn xxyz(self) -> Vec4<T> { Vec4(self.0, self.0, self.1, self.2) }
			pub fn xxyw(self) -> Vec4<T> { Vec4(self.0, self.0, self.1, self.3) }
			pub fn xxzx(self) -> Vec4<T> { Vec4(self.0, self.0, self.2, self.0) }
			pub fn xxzy(self) -> Vec4<T> { Vec4(self.0, self.0, self.2, self.1) }
			pub fn xxzz(self) -> Vec4<T> { Vec4(self.0, self.0, self.2, self.2) }
			pub fn xxzw(self) -> Vec4<T> { Vec4(self.0, self.0, self.2, self.3) }
			pub fn xxwx(self) -> Vec4<T> { Vec4(self.0, self.0, self.3, self.0) }
			pub fn xxwy(self) -> Vec4<T> { Vec4(self.0, self.0, self.3, self.1) }
			pub fn xxwz(self) -> Vec4<T> { Vec4(self.0, self.0, self.3, self.2) }
			pub fn xxww(self) -> Vec4<T> { Vec4(self.0, self.0, self.3, self.3) }
			pub fn xyxx(self) -> Vec4<T> { Vec4(self.0, self.1, self.0, self.0) }
			pub fn xyxy(self) -> Vec4<T> { Vec4(self.0, self.1, self.0, self.1) }
			pub fn xyxz(self) -> Vec4<T> { Vec4(self.0, self.1, self.0, self.2) }
			pub fn xyxw(self) -> Vec4<T> { Vec4(self.0, self.1, self.0, self.3) }
			pub fn xyyx(self) -> Vec4<T> { Vec4(self.0, self.1, self.1, self.0) }
			pub fn xyyy(self) -> Vec4<T> { Vec4(self.0, self.1, self.1, self.1) }
			pub fn xyyz(self) -> Vec4<T> { Vec4(self.0, self.1, self.1, self.2) }
			pub fn xyyw(self) -> Vec4<T> { Vec4(self.0, self.1, self.1, self.3) }
			pub fn xyzx(self) -> Vec4<T> { Vec4(self.0, self.1, self.2, self.0) }
			pub fn xyzy(self) -> Vec4<T> { Vec4(self.0, self.1, self.2, self.1) }
			pub fn xyzz(self) -> Vec4<T> { Vec4(self.0, self.1, self.2, self.2) }
			pub fn xyzw(self) -> Vec4<T> { Vec4(self.0, self.1, self.2, self.3) }
			pub fn xywx(self) -> Vec4<T> { Vec4(self.0, self.1, self.3, self.0) }
			pub fn xywy(self) -> Vec4<T> { Vec4(self.0, self.1, self.3, self.1) }
			pub fn xywz(self) -> Vec4<T> { Vec4(self.0, self.1, self.3, self.2) }
			pub fn xyww(self) -> Vec4<T> { Vec4(self.0, self.1, self.3, self.3) }
			pub fn xzxx(self) -> Vec4<T> { Vec4(self.0, self.2, self.0, self.0) }
			pub fn xzxy(self) -> Vec4<T> { Vec4(self.0, self.2, self.0, self.1) }
			pub fn xzxz(self) -> Vec4<T> { Vec4(self.0, self.2, self.0, self.2) }
			pub fn xzxw(self) -> Vec4<T> { Vec4(self.0, self.2, self.0, self.3) }
			pub fn xzyx(self) -> Vec4<T> { Vec4(self.0, self.2, self.1, self.0) }
			pub fn xzyy(self) -> Vec4<T> { Vec4(self.0, self.2, self.1, self.1) }
			pub fn xzyz(self) -> Vec4<T> { Vec4(self.0, self.2, self.1, self.2) }
			pub fn xzyw(self) -> Vec4<T> { Vec4(self.0, self.2, self.1, self.3) }
			pub fn xzzx(self) -> Vec4<T> { Vec4(self.0, self.2, self.2, self.0) }
			pub fn xzzy(self) -> Vec4<T> { Vec4(self.0, self.2, self.2, self.1) }
			pub fn xzzz(self) -> Vec4<T> { Vec4(self.0, self.2, self.2, self.2) }
			pub fn xzzw(self) -> Vec4<T> { Vec4(self.0, self.2, self.2, self.3) }
			pub fn xzwx(self) -> Vec4<T> { Vec4(self.0, self.2, self.3, self.0) }
			pub fn xzwy(self) -> Vec4<T> { Vec4(self.0, self.2, self.3, self.1) }
			pub fn xzwz(self) -> Vec4<T> { Vec4(self.0, self.2, self.3, self.2) }
			pub fn xzww(self) -> Vec4<T> { Vec4(self.0, self.2, self.3, self.3) }
			pub fn xwxx(self) -> Vec4<T> { Vec4(self.0, self.3, self.0, self.0) }
			pub fn xwxy(self) -> Vec4<T> { Vec4(self.0, self.3, self.0, self.1) }
			pub fn xwxz(self) -> Vec4<T> { Vec4(self.0, self.3, self.0, self.2) }
			pub fn xwxw(self) -> Vec4<T> { Vec4(self.0, self.3, self.0, self.3) }
			pub fn xwyx(self) -> Vec4<T> { Vec4(self.0, self.3, self.1, self.0) }
			pub fn xwyy(self) -> Vec4<T> { Vec4(self.0, self.3, self.1, self.1) }
			pub fn xwyz(self) -> Vec4<T> { Vec4(self.0, self.3, self.1, self.2) }
			pub fn xwyw(self) -> Vec4<T> { Vec4(self.0, self.3, self.1, self.3) }
			pub fn xwzx(self) -> Vec4<T> { Vec4(self.0, self.3, self.2, self.0) }
			pub fn xwzy(self) -> Vec4<T> { Vec4(self.0, self.3, self.2, self.1) }
			pub fn xwzz(self) -> Vec4<T> { Vec4(self.0, self.3, self.2, self.2) }
			pub fn xwzw(self) -> Vec4<T> { Vec4(self.0, self.3, self.2, self.3) }
			pub fn xwwx(self) -> Vec4<T> { Vec4(self.0, self.3, self.3, self.0) }
			pub fn xwwy(self) -> Vec4<T> { Vec4(self.0, self.3, self.3, self.1) }
			pub fn xwwz(self) -> Vec4<T> { Vec4(self.0, self.3, self.3, self.2) }
			pub fn xwww(self) -> Vec4<T> { Vec4(self.0, self.3, self.3, self.3) }
			pub fn yxxx(self) -> Vec4<T> { Vec4(self.1, self.0, self.0, self.0) }
			pub fn yxxy(self) -> Vec4<T> { Vec4(self.1, self.0, self.0, self.1) }
			pub fn yxxz(self) -> Vec4<T> { Vec4(self.1, self.0, self.0, self.2) }
			pub fn yxxw(self) -> Vec4<T> { Vec4(self.1, self.0, self.0, self.3) }
			pub fn yxyx(self) -> Vec4<T> { Vec4(self.1, self.0, self.1, self.0) }
			pub fn yxyy(self) -> Vec4<T> { Vec4(self.1, self.0, self.1, self.1) }
			pub fn yxyz(self) -> Vec4<T> { Vec4(self.1, self.0, self.1, self.2) }
			pub fn yxyw(self) -> Vec4<T> { Vec4(self.1, self.0, self.1, self.3) }
			pub fn yxzx(self) -> Vec4<T> { Vec4(self.1, self.0, self.2, self.0) }
			pub fn yxzy(self) -> Vec4<T> { Vec4(self.1, self.0, self.2, self.1) }
			pub fn yxzz(self) -> Vec4<T> { Vec4(self.1, self.0, self.2, self.2) }
			pub fn yxzw(self) -> Vec4<T> { Vec4(self.1, self.0, self.2, self.3) }
			pub fn yxwx(self) -> Vec4<T> { Vec4(self.1, self.0, self.3, self.0) }
			pub fn yxwy(self) -> Vec4<T> { Vec4(self.1, self.0, self.3, self.1) }
			pub fn yxwz(self) -> Vec4<T> { Vec4(self.1, self.0, self.3, self.2) }
			pub fn yxww(self) -> Vec4<T> { Vec4(self.1, self.0, self.3, self.3) }
			pub fn yyxx(self) -> Vec4<T> { Vec4(self.1, self.1, self.0, self.0) }
			pub fn yyxy(self) -> Vec4<T> { Vec4(self.1, self.1, self.0, self.1) }
			pub fn yyxz(self) -> Vec4<T> { Vec4(self.1, self.1, self.0, self.2) }
			pub fn yyxw(self) -> Vec4<T> { Vec4(self.1, self.1, self.0, self.3) }
			pub fn yyyx(self) -> Vec4<T> { Vec4(self.1, self.1, self.1, self.0) }
			pub fn yyyy(self) -> Vec4<T> { Vec4(self.1, self.1, self.1, self.1) }
			pub fn yyyz(self) -> Vec4<T> { Vec4(self.1, self.1, self.1, self.2) }
			pub fn yyyw(self) -> Vec4<T> { Vec4(self.1, self.1, self.1, self.3) }
			pub fn yyzx(self) -> Vec4<T> { Vec4(self.1, self.1, self.2, self.0) }
			pub fn yyzy(self) -> Vec4<T> { Vec4(self.1, self.1, self.2, self.1) }
			pub fn yyzz(self) -> Vec4<T> { Vec4(self.1, self.1, self.2, self.2) }
			pub fn yyzw(self) -> Vec4<T> { Vec4(self.1, self.1, self.2, self.3) }
			pub fn yywx(self) -> Vec4<T> { Vec4(self.1, self.1, self.3, self.0) }
			pub fn yywy(self) -> Vec4<T> { Vec4(self.1, self.1, self.3, self.1) }
			pub fn yywz(self) -> Vec4<T> { Vec4(self.1, self.1, self.3, self.2) }
			pub fn yyww(self) -> Vec4<T> { Vec4(self.1, self.1, self.3, self.3) }
			pub fn yzxx(self) -> Vec4<T> { Vec4(self.1, self.2, self.0, self.0) }
			pub fn yzxy(self) -> Vec4<T> { Vec4(self.1, self.2, self.0, self.1) }
			pub fn yzxz(self) -> Vec4<T> { Vec4(self.1, self.2, self.0, self.2) }
			pub fn yzxw(self) -> Vec4<T> { Vec4(self.1, self.2, self.0, self.3) }
			pub fn yzyx(self) -> Vec4<T> { Vec4(self.1, self.2, self.1, self.0) }
			pub fn yzyy(self) -> Vec4<T> { Vec4(self.1, self.2, self.1, self.1) }
			pub fn yzyz(self) -> Vec4<T> { Vec4(self.1, self.2, self.1, self.2) }
			pub fn yzyw(self) -> Vec4<T> { Vec4(self.1, self.2, self.1, self.3) }
			pub fn yzzx(self) -> Vec4<T> { Vec4(self.1, self.2, self.2, self.0) }
			pub fn yzzy(self) -> Vec4<T> { Vec4(self.1, self.2, self.2, self.1) }
			pub fn yzzz(self) -> Vec4<T> { Vec4(self.1, self.2, self.2, self.2) }
			pub fn yzzw(self) -> Vec4<T> { Vec4(self.1, self.2, self.2, self.3) }
			pub fn yzwx(self) -> Vec4<T> { Vec4(self.1, self.2, self.3, self.0) }
			pub fn yzwy(self) -> Vec4<T> { Vec4(self.1, self.2, self.3, self.1) }
			pub fn yzwz(self) -> Vec4<T> { Vec4(self.1, self.2, self.3, self.2) }
			pub fn yzww(self) -> Vec4<T> { Vec4(self.1, self.2, self.3, self.3) }
			pub fn ywxx(self) -> Vec4<T> { Vec4(self.1, self.3, self.0, self.0) }
			pub fn ywxy(self) -> Vec4<T> { Vec4(self.1, self.3, self.0, self.1) }
			pub fn ywxz(self) -> Vec4<T> { Vec4(self.1, self.3, self.0, self.2) }
			pub fn ywxw(self) -> Vec4<T> { Vec4(self.1, self.3, self.0, self.3) }
			pub fn ywyx(self) -> Vec4<T> { Vec4(self.1, self.3, self.1, self.0) }
			pub fn ywyy(self) -> Vec4<T> { Vec4(self.1, self.3, self.1, self.1) }
			pub fn ywyz(self) -> Vec4<T> { Vec4(self.1, self.3, self.1, self.2) }
			pub fn ywyw(self) -> Vec4<T> { Vec4(self.1, self.3, self.1, self.3) }
			pub fn ywzx(self) -> Vec4<T> { Vec4(self.1, self.3, self.2, self.0) }
			pub fn ywzy(self) -> Vec4<T> { Vec4(self.1, self.3, self.2, self.1) }
			pub fn ywzz(self) -> Vec4<T> { Vec4(self.1, self.3, self.2, self.2) }
			pub fn ywzw(self) -> Vec4<T> { Vec4(self.1, self.3, self.2, self.3) }
			pub fn ywwx(self) -> Vec4<T> { Vec4(self.1, self.3, self.3, self.0) }
			pub fn ywwy(self) -> Vec4<T> { Vec4(self.1, self.3, self.3, self.1) }
			pub fn ywwz(self) -> Vec4<T> { Vec4(self.1, self.3, self.3, self.2) }
			pub fn ywww(self) -> Vec4<T> { Vec4(self.1, self.3, self.3, self.3) }
			pub fn zxxx(self) -> Vec4<T> { Vec4(self.2, self.0, self.0, self.0) }
			pub fn zxxy(self) -> Vec4<T> { Vec4(self.2, self.0, self.0, self.1) }
			pub fn zxxz(self) -> Vec4<T> { Vec4(self.2, self.0, self.0, self.2) }
			pub fn zxxw(self) -> Vec4<T> { Vec4(self.2, self.0, self.0, self.3) }
			pub fn zxyx(self) -> Vec4<T> { Vec4(self.2, self.0, self.1, self.0) }
			pub fn zxyy(self) -> Vec4<T> { Vec4(self.2, self.0, self.1, self.1) }
			pub fn zxyz(self) -> Vec4<T> { Vec4(self.2, self.0, self.1, self.2) }
			pub fn zxyw(self) -> Vec4<T> { Vec4(self.2, self.0, self.1, self.3) }
			pub fn zxzx(self) -> Vec4<T> { Vec4(self.2, self.0, self.2, self.0) }
			pub fn zxzy(self) -> Vec4<T> { Vec4(self.2, self.0, self.2, self.1) }
			pub fn zxzz(self) -> Vec4<T> { Vec4(self.2, self.0, self.2, self.2) }
			pub fn zxzw(self) -> Vec4<T> { Vec4(self.2, self.0, self.2, self.3) }
			pub fn zxwx(self) -> Vec4<T> { Vec4(self.2, self.0, self.3, self.0) }
			pub fn zxwy(self) -> Vec4<T> { Vec4(self.2, self.0, self.3, self.1) }
			pub fn zxwz(self) -> Vec4<T> { Vec4(self.2, self.0, self.3, self.2) }
			pub fn zxww(self) -> Vec4<T> { Vec4(self.2, self.0, self.3, self.3) }
			pub fn zyxx(self) -> Vec4<T> { Vec4(self.2, self.1, self.0, self.0) }
			pub fn zyxy(self) -> Vec4<T> { Vec4(self.2, self.1, self.0, self.1) }
			pub fn zyxz(self) -> Vec4<T> { Vec4(self.2, self.1, self.0, self.2) }
			pub fn zyxw(self) -> Vec4<T> { Vec4(self.2, self.1, self.0, self.3) }
			pub fn zyyx(self) -> Vec4<T> { Vec4(self.2, self.1, self.1, self.0) }
			pub fn zyyy(self) -> Vec4<T> { Vec4(self.2, self.1, self.1, self.1) }
			pub fn zyyz(self) -> Vec4<T> { Vec4(self.2, self.1, self.1, self.2) }
			pub fn zyyw(self) -> Vec4<T> { Vec4(self.2, self.1, self.1, self.3) }
			pub fn zyzx(self) -> Vec4<T> { Vec4(self.2, self.1, self.2, self.0) }
			pub fn zyzy(self) -> Vec4<T> { Vec4(self.2, self.1, self.2, self.1) }
			pub fn zyzz(self) -> Vec4<T> { Vec4(self.2, self.1, self.2, self.2) }
			pub fn zyzw(self) -> Vec4<T> { Vec4(self.2, self.1, self.2, self.3) }
			pub fn zywx(self) -> Vec4<T> { Vec4(self.2, self.1, self.3, self.0) }
			pub fn zywy(self) -> Vec4<T> { Vec4(self.2, self.1, self.3, self.1) }
			pub fn zywz(self) -> Vec4<T> { Vec4(self.2, self.1, self.3, self.2) }
			pub fn zyww(self) -> Vec4<T> { Vec4(self.2, self.1, self.3, self.3) }
			pub fn zzxx(self) -> Vec4<T> { Vec4(self.2, self.2, self.0, self.0) }
			pub fn zzxy(self) -> Vec4<T> { Vec4(self.2, self.2, self.0, self.1) }
			pub fn zzxz(self) -> Vec4<T> { Vec4(self.2, self.2, self.0, self.2) }
			pub fn zzxw(self) -> Vec4<T> { Vec4(self.2, self.2, self.0, self.3) }
			pub fn zzyx(self) -> Vec4<T> { Vec4(self.2, self.2, self.1, self.0) }
			pub fn zzyy(self) -> Vec4<T> { Vec4(self.2, self.2, self.1, self.1) }
			pub fn zzyz(self) -> Vec4<T> { Vec4(self.2, self.2, self.1, self.2) }
			pub fn zzyw(self) -> Vec4<T> { Vec4(self.2, self.2, self.1, self.3) }
			pub fn zzzx(self) -> Vec4<T> { Vec4(self.2, self.2, self.2, self.0) }
			pub fn zzzy(self) -> Vec4<T> { Vec4(self.2, self.2, self.2, self.1) }
			pub fn zzzz(self) -> Vec4<T> { Vec4(self.2, self.2, self.2, self.2) }
			pub fn zzzw(self) -> Vec4<T> { Vec4(self.2, self.2, self.2, self.3) }
			pub fn zzwx(self) -> Vec4<T> { Vec4(self.2, self.2, self.3, self.0) }
			pub fn zzwy(self) -> Vec4<T> { Vec4(self.2, self.2, self.3, self.1) }
			pub fn zzwz(self) -> Vec4<T> { Vec4(self.2, self.2, self.3, self.2) }
			pub fn zzww(self) -> Vec4<T> { Vec4(self.2, self.2, self.3, self.3) }
			pub fn zwxx(self) -> Vec4<T> { Vec4(self.2, self.3, self.0, self.0) }
			pub fn zwxy(self) -> Vec4<T> { Vec4(self.2, self.3, self.0, self.1) }
			pub fn zwxz(self) -> Vec4<T> { Vec4(self.2, self.3, self.0, self.2) }
			pub fn zwxw(self) -> Vec4<T> { Vec4(self.2, self.3, self.0, self.3) }
			pub fn zwyx(self) -> Vec4<T> { Vec4(self.2, self.3, self.1, self.0) }
			pub fn zwyy(self) -> Vec4<T> { Vec4(self.2, self.3, self.1, self.1) }
			pub fn zwyz(self) -> Vec4<T> { Vec4(self.2, self.3, self.1, self.2) }
			pub fn zwyw(self) -> Vec4<T> { Vec4(self.2, self.3, self.1, self.3) }
			pub fn zwzx(self) -> Vec4<T> { Vec4(self.2, self.3, self.2, self.0) }
			pub fn zwzy(self) -> Vec4<T> { Vec4(self.2, self.3, self.2, self.1) }
			pub fn zwzz(self) -> Vec4<T> { Vec4(self.2, self.3, self.2, self.2) }
			pub fn zwzw(self) -> Vec4<T> { Vec4(self.2, self.3, self.2, self.3) }
			pub fn zwwx(self) -> Vec4<T> { Vec4(self.2, self.3, self.3, self.0) }
			pub fn zwwy(self) -> Vec4<T> { Vec4(self.2, self.3, self.3, self.1) }
			pub fn zwwz(self) -> Vec4<T> { Vec4(self.2, self.3, self.3, self.2) }
			pub fn zwww(self) -> Vec4<T> { Vec4(self.2, self.3, self.3, self.3) }
			pub fn wxxx(self) -> Vec4<T> { Vec4(self.3, self.0, self.0, self.0) }
			pub fn wxxy(self) -> Vec4<T> { Vec4(self.3, self.0, self.0, self.1) }
			pub fn wxxz(self) -> Vec4<T> { Vec4(self.3, self.0, self.0, self.2) }
			pub fn wxxw(self) -> Vec4<T> { Vec4(self.3, self.0, self.0, self.3) }
			pub fn wxyx(self) -> Vec4<T> { Vec4(self.3, self.0, self.1, self.0) }
			pub fn wxyy(self) -> Vec4<T> { Vec4(self.3, self.0, self.1, self.1) }
			pub fn wxyz(self) -> Vec4<T> { Vec4(self.3, self.0, self.1, self.2) }
			pub fn wxyw(self) -> Vec4<T> { Vec4(self.3, self.0, self.1, self.3) }
			pub fn wxzx(self) -> Vec4<T> { Vec4(self.3, self.0, self.2, self.0) }
			pub fn wxzy(self) -> Vec4<T> { Vec4(self.3, self.0, self.2, self.1) }
			pub fn wxzz(self) -> Vec4<T> { Vec4(self.3, self.0, self.2, self.2) }
			pub fn wxzw(self) -> Vec4<T> { Vec4(self.3, self.0, self.2, self.3) }
			pub fn wxwx(self) -> Vec4<T> { Vec4(self.3, self.0, self.3, self.0) }
			pub fn wxwy(self) -> Vec4<T> { Vec4(self.3, self.0, self.3, self.1) }
			pub fn wxwz(self) -> Vec4<T> { Vec4(self.3, self.0, self.3, self.2) }
			pub fn wxww(self) -> Vec4<T> { Vec4(self.3, self.0, self.3, self.3) }
			pub fn wyxx(self) -> Vec4<T> { Vec4(self.3, self.1, self.0, self.0) }
			pub fn wyxy(self) -> Vec4<T> { Vec4(self.3, self.1, self.0, self.1) }
			pub fn wyxz(self) -> Vec4<T> { Vec4(self.3, self.1, self.0, self.2) }
			pub fn wyxw(self) -> Vec4<T> { Vec4(self.3, self.1, self.0, self.3) }
			pub fn wyyx(self) -> Vec4<T> { Vec4(self.3, self.1, self.1, self.0) }
			pub fn wyyy(self) -> Vec4<T> { Vec4(self.3, self.1, self.1, self.1) }
			pub fn wyyz(self) -> Vec4<T> { Vec4(self.3, self.1, self.1, self.2) }
			pub fn wyyw(self) -> Vec4<T> { Vec4(self.3, self.1, self.1, self.3) }
			pub fn wyzx(self) -> Vec4<T> { Vec4(self.3, self.1, self.2, self.0) }
			pub fn wyzy(self) -> Vec4<T> { Vec4(self.3, self.1, self.2, self.1) }
			pub fn wyzz(self) -> Vec4<T> { Vec4(self.3, self.1, self.2, self.2) }
			pub fn wyzw(self) -> Vec4<T> { Vec4(self.3, self.1, self.2, self.3) }
			pub fn wywx(self) -> Vec4<T> { Vec4(self.3, self.1, self.3, self.0) }
			pub fn wywy(self) -> Vec4<T> { Vec4(self.3, self.1, self.3, self.1) }
			pub fn wywz(self) -> Vec4<T> { Vec4(self.3, self.1, self.3, self.2) }
			pub fn wyww(self) -> Vec4<T> { Vec4(self.3, self.1, self.3, self.3) }
			pub fn wzxx(self) -> Vec4<T> { Vec4(self.3, self.2, self.0, self.0) }
			pub fn wzxy(self) -> Vec4<T> { Vec4(self.3, self.2, self.0, self.1) }
			pub fn wzxz(self) -> Vec4<T> { Vec4(self.3, self.2, self.0, self.2) }
			pub fn wzxw(self) -> Vec4<T> { Vec4(self.3, self.2, self.0, self.3) }
			pub fn wzyx(self) -> Vec4<T> { Vec4(self.3, self.2, self.1, self.0) }
			pub fn wzyy(self) -> Vec4<T> { Vec4(self.3, self.2, self.1, self.1) }
			pub fn wzyz(self) -> Vec4<T> { Vec4(self.3, self.2, self.1, self.2) }
			pub fn wzyw(self) -> Vec4<T> { Vec4(self.3, self.2, self.1, self.3) }
			pub fn wzzx(self) -> Vec4<T> { Vec4(self.3, self.2, self.2, self.0) }
			pub fn wzzy(self) -> Vec4<T> { Vec4(self.3, self.2, self.2, self.1) }
			pub fn wzzz(self) -> Vec4<T> { Vec4(self.3, self.2, self.2, self.2) }
			pub fn wzzw(self) -> Vec4<T> { Vec4(self.3, self.2, self.2, self.3) }
			pub fn wzwx(self) -> Vec4<T> { Vec4(self.3, self.2, self.3, self.0) }
			pub fn wzwy(self) -> Vec4<T> { Vec4(self.3, self.2, self.3, self.1) }
			pub fn wzwz(self) -> Vec4<T> { Vec4(self.3, self.2, self.3, self.2) }
			pub fn wzww(self) -> Vec4<T> { Vec4(self.3, self.2, self.3, self.3) }
			pub fn wwxx(self) -> Vec4<T> { Vec4(self.3, self.3, self.0, self.0) }
			pub fn wwxy(self) -> Vec4<T> { Vec4(self.3, self.3, self.0, self.1) }
			pub fn wwxz(self) -> Vec4<T> { Vec4(self.3, self.3, self.0, self.2) }
			pub fn wwxw(self) -> Vec4<T> { Vec4(self.3, self.3, self.0, self.3) }
			pub fn wwyx(self) -> Vec4<T> { Vec4(self.3, self.3, self.1, self.0) }
			pub fn wwyy(self) -> Vec4<T> { Vec4(self.3, self.3, self.1, self.1) }
			pub fn wwyz(self) -> Vec4<T> { Vec4(self.3, self.3, self.1, self.2) }
			pub fn wwyw(self) -> Vec4<T> { Vec4(self.3, self.3, self.1, self.3) }
			pub fn wwzx(self) -> Vec4<T> { Vec4(self.3, self.3, self.2, self.0) }
			pub fn wwzy(self) -> Vec4<T> { Vec4(self.3, self.3, self.2, self.1) }
			pub fn wwzz(self) -> Vec4<T> { Vec4(self.3, self.3, self.2, self.2) }
			pub fn wwzw(self) -> Vec4<T> { Vec4(self.3, self.3, self.2, self.3) }
			pub fn wwwx(self) -> Vec4<T> { Vec4(self.3, self.3, self.3, self.0) }
			pub fn wwwy(self) -> Vec4<T> { Vec4(self.3, self.3, self.3, self.1) }
			pub fn wwwz(self) -> Vec4<T> { Vec4(self.3, self.3, self.3, self.2) }
			pub fn wwww(self) -> Vec4<T> { Vec4(self.3, self.3, self.3, self.3) }
		}
    };
}
