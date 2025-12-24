use std::io;

use io::BufWriter;
use io::Write;

use io::Read;

use rawzip::ZipArchive;
use rawzip::ZipArchiveEntryWayfinder;
use rawzip::ZipSliceArchive;
use rawzip::ZipSliceEntries;
use rawzip::ZipSliceEntry;

pub struct Archive<'a>(pub ZipSliceArchive<&'a [u8]>);

impl<'a> Archive<'a> {
    pub fn way2entry(
        &self,
        finder: ZipArchiveEntryWayfinder,
    ) -> Result<ZipSliceEntry<'_>, rawzip::Error> {
        self.0.get_entry(finder)
    }

    pub fn way2entry_raw(&self, finder: ZipArchiveEntryWayfinder) -> Result<&[u8], rawzip::Error> {
        self.way2entry(finder).map(|ent| ent.data())
    }

    pub fn entries(&self) -> ZipSliceEntries<'_> {
        self.0.entries()
    }

    pub fn way1st(&self) -> Result<Option<ZipArchiveEntryWayfinder>, rawzip::Error> {
        let mut ents = self.entries();
        let nxt: Option<_> = ents.next_entry()?;
        Ok(nxt.map(|rec| rec.wayfinder()))
    }

    pub fn entry1st_raw(&self) -> Result<Option<&[u8]>, rawzip::Error> {
        let oway: Option<_> = self.way1st()?;
        match oway {
            None => Ok(None),
            Some(way) => self.way2entry_raw(way).map(Some),
        }
    }
}

pub fn slice2archive(s: &[u8]) -> Result<Archive<'_>, rawzip::Error> {
    ZipArchive::from_slice(s).map(Archive)
}

pub fn slice2archive2entry1st_raw2wtr<W>(s: &[u8], mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    let a: Archive = slice2archive(s).map_err(io::Error::other)?;
    let oraw: Option<&[u8]> = a.entry1st_raw().map_err(io::Error::other)?;
    if let Some(raw) = oraw {
        wtr.write_all(raw)?;
        wtr.flush()?;
    }
    Ok(())
}

pub fn slice2archive2entry1st_raw2stdout(s: &[u8]) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    let bw = BufWriter::new(&mut ol);
    slice2archive2entry1st_raw2wtr(s, bw)?;
    ol.flush()
}

pub fn stdin2slice2archive2entry1st_raw2stdout(limit: u64) -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let mut taken = il.take(limit);

    let mut buf: Vec<u8> = vec![];
    taken.read_to_end(&mut buf)?;
    slice2archive2entry1st_raw2stdout(&buf)
}
