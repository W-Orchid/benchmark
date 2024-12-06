use std::fmt::Display;
use std::time::Duration;
use std::cmp::{min, max};

use womscp_lib::womscp::Request;

use super::*;


pub struct RequestBenchmark {
    pub id :u32,
    pub elapsed :Duration,
    pub request :Request,
    pub response :Result<(), ResponseError>
}

pub struct Results {
    pub min_response_time :Duration,
    pub max_response_time :Duration,
    pub avg_response_time :Duration,
    total_response_times  :Duration,

    user_defined_failure_rate :u8,

    pub ok_responses :u32,
    pub not_ready_errors :u32,
    pub version_errors :u32,
    pub unrecognized_errors :u32,
    pub tcp_errors :u32,
    pub database_errors :u32,
    total_responses :u32,
    concurrent_requests :u16
}

impl Results {
    pub fn new(total_requests_made :u32, concurrent_req :u16, failure_rate :u8) -> Self {
        Results {
            min_response_time: Duration::from_nanos(100),
            max_response_time: Duration::from_secs(0),
            avg_response_time: Duration::from_nanos(0),
            total_response_times: Duration::from_nanos(0),

            user_defined_failure_rate: failure_rate,

            ok_responses: 0,
            not_ready_errors: 0,
            version_errors: 0,
            unrecognized_errors: 0,
            tcp_errors: 0,
            database_errors: 0,
            total_responses: total_requests_made,
            concurrent_requests: concurrent_req
        }
    }

    pub fn update(&mut self, benchmark :RequestBenchmark) {
        self.min_response_time = min(self.min_response_time, benchmark.elapsed);
        self.max_response_time = max(self.max_response_time,benchmark.elapsed);

        self.total_response_times += benchmark.elapsed;

        self.avg_response_time = self.total_response_times / self.total_responses;

        match benchmark.response {
            Ok(_) => self.ok_responses += 1,
            Err(ResponseError::NotReady) => self.not_ready_errors += 1,
            Err(ResponseError::Version) => self.version_errors += 1,
            Err(ResponseError::Unrecognised) => self.unrecognized_errors += 1,
            Err(ResponseError::Tcp) => self.tcp_errors += 1,
            Err(ResponseError::Database) => self.database_errors += 1
        };
    }
}


impl Display for Results {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            " *** BENCHMARK RESULTS *** 
    Total Requests                     = {:>15}
    Concurrent Requests                = {:>15}
    User-Defined Failure Rate          = {:>15} %
    Actual Failure Rate                = {:>15} %
        Server Not Ready Errors        = {:>15}
        Version Errors                 = {:>15} (all artificial errors were Version errors)
        Unrecognised Errors            = {:>15}
        TCP Errors                     = {:>15}
        Database Errors                = {:>15}


    Min. Response Time                 = {:>15} ns
    Max. Response Time                 = {:>15} ns
    Avg. Response Time                 = {:>15} ns

 *************************",
    self.total_responses,
    self.concurrent_requests,
    self.user_defined_failure_rate as f32,
    (self.total_responses - self.ok_responses) as f32 / self.total_responses as f32 * 100.0,
    self.not_ready_errors,
    self.version_errors,
    self.unrecognized_errors,
    self.tcp_errors,
    self.database_errors,
    self.min_response_time.as_nanos(),
    self.max_response_time.as_nanos(),
    self.avg_response_time.as_nanos()
)
    }
}
