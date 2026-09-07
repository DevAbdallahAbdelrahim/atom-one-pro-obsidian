.PHONY: build dev test clean

# بناء الأداة وتجميع الثيم مرة واحدة
build:
	cargo run --release

# تشغيل المراقبة الحية والتحديث الفوري
dev:
	cargo run --release -- --watch

# تشغيل الاختبارات الآلية من مجلد tests/
test:
	cargo test --release

# تنظيف مجلد target وملف theme.css
clean:
	cargo clean
	rm -f theme.css
