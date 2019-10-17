#!/bin/sh

# If using normal root, avoid changing anything.
if [ -z "$RPM_BUILD_ROOT" -o "$RPM_BUILD_ROOT" = "/" ]; then
	exit 0
fi

cd $RPM_BUILD_ROOT

# Compress man pages
COMPRESS="gzip -9 -n"
COMPRESS_EXT=

for d in ./usr/man/man* ./usr/man/*/man* ./usr/info \
	./usr/share/man/man* ./usr/share/man/*/man* ./usr/share/info \
	./usr/kerberos/man ./usr/X11R6/man/man* ./usr/lib/perl5/man/man* \
	./usr/share/doc/*/man/man* ./usr/lib/*/man/man*
do
    [ -d $d ] || continue
    for f in `find $d -type f`
    do
        [ -f "$f" ] || continue
	[ "`basename $f`" = "dir" ] && continue

	case "$f" in
	 *.Z) gunzip -f $f; b=`echo $f | sed -e 's/\.Z$//'`;;
	 *.gz) gunzip -f $f; b=`echo $f | sed -e 's/\.gz$//'`;;
	 *.bz2) bunzip2 -f $f; b=`echo $f | sed -e 's/\.bz2$//'`;;
	 *) b=$f;;
	esac
    done

    for f in `find $d -type l`
    do
	l=`ls -l $f | sed -e 's/.* -> //' -e 's/\.gz$//' -e 's/\.bz2$//' -e 's/\.Z$//'`
	rm -f $f
	b=`echo $f | sed -e 's/\.gz$//' -e 's/\.bz2$//' -e 's/\.Z$//'`
	ln -sf $l$COMPRESS_EXT $b$COMPRESS_EXT
    done
done
