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

    pub ok_responses :u32,
    pub not_ready_errors :u32,
    pub version_errors :u32,
    pub unrecognized_errors :u32,
    pub tcp_errors :u32,
    pub database_errors :u32,
    total_responses :u32
}

impl Results {
    pub fn new(total_requests_made :u32) -> Self {
        Results {
            min_response_time: Duration::from_nanos(0),
            max_response_time: Duration::from_secs(100),
            avg_response_time: Duration::from_nanos(0),

            total_response_times: Duration::from_nanos(0),

            ok_responses: 0,
            not_ready_errors: 0,
            version_errors: 0,
            unrecognized_errors: 0,
            tcp_errors: 0,
            database_errors: 0,
            total_responses: total_requests_made
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
