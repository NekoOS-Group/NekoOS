pub trait ScheResource {
    
}

pub trait SchePreemptor {
    
}

pub trait Scheduler<R, P> 
    where R: ScheResource,
          P: SchePreemptor
{
    
}