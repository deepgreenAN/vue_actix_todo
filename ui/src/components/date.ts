const convert_datetime = (utc_datetime_str: string):string => {
  if (utc_datetime_str!="") {
    const local_dt = new Date(Date.parse(utc_datetime_str+"Z"));
    const local_dt_str = `${local_dt.getFullYear()}-${local_dt.getMonth()+1}-${local_dt.getDate()}`
    +" "+ `${local_dt.getHours()}:${local_dt.getMinutes()+1}:${local_dt.getSeconds()+1}`;
    return local_dt_str
  } else {
    return ""
  }
};

//const convert_datetime = (utc_datetime_str: string):string => {return utc_datetime_str};

export {
  convert_datetime
}