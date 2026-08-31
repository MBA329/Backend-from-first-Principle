import nock from 'nock';
import axios from 'axios';

describe('FetchRate', () => {
    it('fetches rate from external API', async () => {
        nock('https://api.example.com')
            .get('/rates')
            .reply(200, { usd_inr: 83.2 });

        const response = await axios.get('https://api.example.com/rates');
        expect(response.data.usd_inr).toBe(83.2);
    });
});
