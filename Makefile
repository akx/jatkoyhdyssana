sanat.txt: data/kotus_sanat.txt data/joukahainen.txt
	cat $? | sed 's/=//g' | sort | uniq > $@

data/joukahainen.txt: data/joukahainen.xml.zst
	zstdcat $< | sed -ne 's,.*<form>\(.*\)</form>.*,\1,p' | sort > $@

data/joukahainen.xml.zst:
	curl -L https://joukahainen.puimula.org/sanastot/joukahainen.xml.gz | gunzip | zstd -T0 -11 -o $@

data/kotus_sanat.txt: data/kotus-sanalista/kotus-sanalista_v1.xml.zst
	zstdcat $< | sed -ne 's,.*<s>\(.*\)</s>.*,\1,p' | sort > $@
