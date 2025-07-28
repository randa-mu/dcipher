//! Point-to-point communication protocol

use async_trait::async_trait;
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use libp2p::request_response;
use std::io;

pub(super) const POINT_TO_POINT_PROTOCOL: &str = "/dcipher/point2point/v1";

#[derive(Default, Clone)]
pub(super) struct DcipherPoint2PointMessageCodec;

#[async_trait]
impl request_response::Codec for DcipherPoint2PointMessageCodec {
    type Protocol = &'static str;
    type Request = Vec<u8>;
    type Response = ();

    /// Reads a request from the given I/O stream according to the
    /// negotiated protocol.
    async fn read_request<T>(
        &mut self,
        protocol: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        if *protocol != POINT_TO_POINT_PROTOCOL {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "unsupported protocol",
            ))?
        }

        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;

        Ok(buf)
    }

    /// Reads a response from the given I/O stream according to the
    /// negotiated protocol.
    async fn read_response<T>(
        &mut self,
        protocol: &Self::Protocol,
        _io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        if *protocol != POINT_TO_POINT_PROTOCOL {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "unsupported protocol",
            ))?
        }

        Ok(())
    }

    /// Writes a request to the given I/O stream according to the
    /// negotiated protocol.
    async fn write_request<T>(
        &mut self,
        protocol: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        if *protocol != POINT_TO_POINT_PROTOCOL {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "unsupported protocol",
            ))?
        }

        io.write_all(&req).await
    }

    /// Writes a response to the given I/O stream according to the
    /// negotiated protocol.
    async fn write_response<T>(
        &mut self,
        protocol: &Self::Protocol,
        _io: &mut T,
        _res: Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        if *protocol != POINT_TO_POINT_PROTOCOL {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "unsupported protocol",
            ))?
        }

        Ok(())
    }
}
