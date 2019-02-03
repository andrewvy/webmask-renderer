b.prototype.nK = function (b, c) {
var d = this;
this.Hs(b, c, function (b) {
d.Sn = c;
var l = new DataView(b);
d.ue || (d.ue = w.Jo());
d.Xc = {
tag: d.ue.decode(b.slice(0, 4)),
version: l.getInt32(4),
kI: l.getInt8(8),
vU: l.getInt32(8),
Ly: l.getInt32(12)
};
b = 16 * d.Xc.Ly;
'MASK' === d.Xc.tag && 2 === d.Xc.kI && d.MK(c, c + b)
})
};
b.prototype.MK = function (b, c) {
var d = this;
this.Hs(b, c, function (b) {
d.Rz = !1;
d.Sn = c;
b = new DataView(b);
for (var l = 0, f = 0; f < d.Xc.Ly; f++) if (0 === b.getInt32(l) && 0 === b.getInt32(l + 8)) {
var g = b.getInt32(l + 4),
u = b.getInt32(l + 12),
l = l + 16;
d.Zb.list.push({
time: g,
offset: u
})
}
b = d.Zb.list.length;
l = 1000 * d.b.duration();
l > d.Zb.list[b - 1].time && (d.Zb.list.push({
time: l,
offset: - 1
}), b++);
d.Aj = b > d.Aj ? d.Aj : b;
d.pA(1000);
d.animate || d.Up()
})
};
b.prototype.pA = function (b) {
for (var d = this, c, f, g = 0, h = this.Zb.list.length; g < h; ) {
var k = this.Zb.list[g],
m = this.Zb.list[g + this.Aj];
f = g + this.Aj;
m || (f = h - 1, m = this.Zb.list[f]);
if (k.time <= b && b < m.time) {
c = g;
break
}
g += this.Aj
}
if ('undefined' !== typeof c) {
b = this.Zb.list[c];
var g = this.Zb.list[f],
h = b.offset,
n = g.offset;
this.up = {
start: b.time,
end: g.time
};
this.Hs(h, n, function (b) {
d.Sn = n;
for (var l = d.Zb.list[c].offset, g = c; g < f; g++) {
var u = d.Zb.list[g + 1].offset;
u = - 1 === u ? b.slice(d.Zb.list[g].offset - l)  : b.slice(d.Zb.list[g].offset - l, u - l);
u = Jf.a.ft(u).buffer;
d.parse(0 === c && 0 === g ? 0 : d.Zb.list[g].time, d.Zb.list[g + 1].time, u)
}
d.up = {
start: 0,
end: 0
};
d.hB(d.Zb.Xr[0].data[0].data)
})
}

b.prototype.parse = function (b, c, f) {
var d = [
];
this.ue || (this.ue = w.Jo());
for (var l, g, u = new DataView(f), h = 0, k = u.byteLength; h < k; ) l = u.getInt32(h),
g = u.getInt32(h + 8),

d.push({
  time: g,
  data: this.ue.decode(f.slice(h + 12, h + 12 + l))
}),

h = h + 12 + l;
this.Zb.Xr.push({
start: b,
end: c,
data: d
})
};
