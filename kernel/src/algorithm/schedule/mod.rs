pub trait ScheResource {
    
}

pub trait SchePreemptor {
    
}

pub trait SchePriority {

}

pub trait Scheduler<R, P> 
    where R: ScheResource,
          P: SchePreemptor + SchePriority
{
    
}

