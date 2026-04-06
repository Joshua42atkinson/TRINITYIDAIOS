import React, { useState, useRef, useEffect, useMemo } from 'react';
import { marked } from 'marked';

function splitIntoPages(markdown, maxChars = 1000) {
    if (!markdown || markdown.trim().length === 0) return [''];
    const rawBlocks = markdown.split(/\n\n+/).filter(b => b.trim().length > 0);
    const pages = [];
    let currentBlocks = [];
    let currentLen = 0;

    for (let i = 0; i < rawBlocks.length; i++) {
        const block = rawBlocks[i].trim();
        const isTable = block.includes('|') && block.includes('---');
        const isHeading = block.startsWith('# ');
        const isImage = block.startsWith('![');

        if (isHeading) {
            // Force a page break before top-level headings
            if (currentBlocks.length > 0) {
                pages.push(currentBlocks.join('\n\n'));
                currentBlocks = [];
                currentLen = 0;
            }
        }

        if (isTable || isImage) {
            if (currentBlocks.length > 0) {
                pages.push(currentBlocks.join('\n\n'));
                currentBlocks = [];
                currentLen = 0;
            }
            pages.push(block);
            continue;
        }

        if (currentLen + block.length > maxChars && currentBlocks.length > 0) {
            pages.push(currentBlocks.join('\n\n'));
            currentBlocks = [block];
            currentLen = block.length;
        } else {
            currentBlocks.push(block);
            currentLen += block.length;
        }
    }
    if (currentBlocks.length > 0) pages.push(currentBlocks.join('\n\n'));
    return pages.length > 0 ? pages : [''];
}

export default function FieldManualViewer({ onBack }) {
    const [spreadIndex, setSpreadIndex] = useState(0);
    const [flipAnime, setFlipAnime] = useState('');
    const [pagesContent, setPagesContent] = useState([]);
    const flipTimer = useRef(null);

    useEffect(() => {
        fetch('/docs/ASK_PETE_FIELD_MANUAL.md')
            .then(res => res.text())
            .then(text => {
                const chunks = splitIntoPages(text, 1000);
                
                const pageObjs = chunks.map((chunk, idx) => {
                    const isTitle = chunk.startsWith('# **ASK PETE');
                    return {
                        content: chunk,
                        globalPage: idx + 1,
                        isTitlePage: isTitle,
                    };
                });
                setPagesContent(pageObjs);
            })
            .catch(err => console.error(err));
    }, []);

    const spreads = useMemo(() => {
        const spreadArray = [];
        let i = 0;
        while (i < pagesContent.length) {
            const page = pagesContent[i];
            
            // If it's the title page, we want it on the right to simulate a book cover
            if (page.isTitlePage) {
                spreadArray.push([ null, page ]);
                i += 1;
            } else {
                const leftPage = page;
                const rightPage = i + 1 < pagesContent.length ? pagesContent[i + 1] : null;
                spreadArray.push([ leftPage, rightPage ]);
                i += rightPage ? 2 : 1;
            }
        }
        return spreadArray;
    }, [pagesContent]);

    const handleFlip = (direction, targetIdx) => {
        setFlipAnime(direction);
        if (flipTimer.current) clearTimeout(flipTimer.current);
        flipTimer.current = setTimeout(() => setFlipAnime(''), 600);
        setTimeout(() => setSpreadIndex(targetIdx), 300);
    };

    const nextSlide = () => { if (spreadIndex < spreads.length - 1) handleFlip('flip-forward', spreadIndex + 1); };
    const prevSlide = () => { if (spreadIndex > 0) handleFlip('flip-backward', spreadIndex - 1); };

    const renderPageContent = (page) => {
        if (!page) return null;
        let htmlContent = marked.parse(page.content);
        
        // Auto-fix image paths based on local serving
        htmlContent = htmlContent.replace(/src="\/images\//g, 'src="/docs/images/');

        return (
            <div className={`dnd-markdown ${page.isTitlePage ? 'title-page-markdown' : ''}`}>
                <div dangerouslySetInnerHTML={{ __html: htmlContent }} />
            </div>
        );
    };

    const currentSpread = spreads[spreadIndex] || [null, null];
    const leftPage = currentSpread[0];
    const rightPage = currentSpread[1];

    if (pagesContent.length === 0) {
        return (
            <div className="handbook-container">
                <button onClick={onBack} className="stow-btn">← Stow Manual</button>
                <div style={{ color: '#CFB991', fontFamily: '"Inter", sans-serif' }}>Retrieving Field Manual from servers...</div>
            </div>
        );
    }

    return (
        <div className="handbook-container">
            <button onClick={onBack} className="stow-btn">← Stow Manual</button>
            <div className="page-counter">Spread {spreadIndex + 1} of {spreads.length}</div>

            <div className={`book-spread ${flipAnime}`}>
                <div className="book-spine" />

                {/* Left Page */}
                <div className="book-page left-page content-bg">
                    {renderPageContent(leftPage)}
                    {leftPage && <div className="page-watermark-left">{leftPage.globalPage}</div>}
                </div>

                {/* Right Page */}
                <div className="book-page right-page content-bg">
                    {renderPageContent(rightPage)}
                    {rightPage && <div className="page-watermark-right">{rightPage.globalPage}</div>}
                </div>
            </div>

            <div className="audio-bar" style={{ justifyContent: 'center' }}>
                <div className="nav-group">
                    <button onClick={prevSlide} disabled={spreadIndex === 0}
                        className={`nav-btn ${spreadIndex === 0 ? 'nav-btn--disabled' : ''}`}>
                        ◀ Turn Back
                    </button>
                    <button onClick={nextSlide} disabled={spreadIndex === spreads.length - 1}
                        className={`nav-btn ${spreadIndex === spreads.length - 1 ? 'nav-btn--disabled' : ''}`}>
                        Turn Page ▶
                    </button>
                </div>
            </div>

            <style dangerouslySetInnerHTML={{__html: `
                @import url('https://fonts.googleapis.com/css2?family=Cinzel:wght@400;700;900&family=Inter:wght@400;500;600&display=swap');

                .handbook-container {
                    grid-column: 1 / -1; grid-row: 2;
                    display: flex; flex-direction: column;
                    align-items: center; justify-content: center;
                    background: #0a0a0a;
                    height: 100%; width: 100%; overflow: hidden;
                    padding: 16px; position: relative;
                }

                .stow-btn {
                    position: absolute; top: 16px; left: 32px; z-index: 50;
                    padding: 8px 16px; background: rgba(15,12,8,0.9);
                    border: 1px solid #CFB991; color: #CFB991;
                    font-family: 'Cinzel', serif; font-size: 13px; cursor: pointer;
                    border-radius: 4px; transition: all 0.2s;
                }
                .stow-btn:hover { background: rgba(207,185,145,0.15); }
                .page-counter {
                    position: absolute; top: 16px; right: 32px; z-index: 50;
                    color: #CFB991; font-family: 'Inter', sans-serif; font-size: 13px;
                    background: rgba(15,12,8,0.9); padding: 6px 16px;
                    border-radius: 4px; border: 1px solid rgba(207,185,145,0.3);
                }

                .book-spread {
                    display: flex; width: 100%; max-width: 1400px; height: calc(100% - 80px);
                    background: #2a2015; 
                    box-shadow: 0 30px 60px rgba(0,0,0,0.8), inset 0 0 80px rgba(139,115,85,0.2);
                    border-radius: 4px 8px 8px 4px; border: 2px solid #5a4b3c;
                    position: relative; overflow: hidden; transition: opacity 0.6s ease, transform 0.6s ease;
                }
                .flip-forward { opacity: 0; transform: translateX(80px); }
                .flip-backward { opacity: 0; transform: translateX(-80px); }

                .book-spine {
                    position: absolute; top: 0; bottom: 0; left: 50%; width: 40px; margin-left: -20px;
                    background: linear-gradient(to right, rgba(0,0,0,0.4), rgba(0,0,0,0.8) 50%, rgba(0,0,0,0.4));
                    z-index: 10; pointer-events: none;
                }

                .book-page {
                    flex: 1; position: relative; z-index: 5;
                    overflow: auto; display: flex; flex-direction: column;
                }
                
                .left-page {
                    padding: 36px 52px 28px 44px;
                    box-shadow: inset -25px 0 25px rgba(0,0,0,0.15);
                }
                
                .right-page {
                    padding: 36px 44px 28px 52px;
                    box-shadow: inset 25px 0 25px rgba(0,0,0,0.15);
                }
                
                .content-bg {
                    background: linear-gradient(135deg, #f4ebd8 0%, #ede2cc 60%, #e8d9bf 100%);
                }

                .page-watermark-left {
                    position: absolute; bottom: 12px; left: 20px;
                    font-family: 'Cinzel', serif; font-size: 14px; color: rgba(139,115,85,0.4);
                }
                .page-watermark-right {
                    position: absolute; bottom: 12px; right: 20px;
                    font-family: 'Cinzel', serif; font-size: 14px; color: rgba(139,115,85,0.4);
                }

                .dnd-markdown {
                    font-family: 'Inter', sans-serif;
                    color: #2a2015; font-size: 14px; line-height: 1.6;
                }
                .dnd-markdown > div > p:first-of-type::first-letter {
                    font-family: 'Cinzel', serif; font-size: 3.2em;
                    float: left; margin: 6px 6px 0 -2px;
                    line-height: 0.8; color: #5A1B1B;
                    text-shadow: 1px 1px 2px rgba(0,0,0,0.15);
                }
                
                .title-page-markdown > div > p:first-of-type::first-letter {
                    font-size: inherit; float: none; margin: 0; line-height: inherit; text-shadow: none;
                }

                .dnd-markdown p { margin-bottom: 12px; text-align: justify; }
                .dnd-markdown h1, .dnd-markdown h2, .dnd-markdown h3 {
                    font-family: 'Cinzel', serif; color: #5A1B1B;
                    margin-top: 16px; margin-bottom: 8px;
                    border-bottom: 1px solid rgba(207,185,145,0.4);
                }

                .dnd-markdown img {
                    max-width: 100%; border-radius: 4px;
                    box-shadow: 0 4px 10px rgba(0,0,0,0.2);
                    margin: 16px auto; display: block;
                    border: 2px solid #CFB991;
                }

                .dnd-markdown blockquote {
                    background: linear-gradient(135deg, rgba(207,185,145,0.12), rgba(207,185,145,0.25));
                    border: 2px solid #CFB991; border-radius: 4px;
                    padding: 14px 16px; margin: 12px 0;
                    box-shadow: inset 0 0 15px rgba(0,0,0,0.04), 0 3px 8px rgba(0,0,0,0.08);
                    position: relative;
                }
                .dnd-markdown blockquote::before {
                    content: ''; position: absolute; top: -3px; left: -3px;
                    width: 14px; height: 14px;
                    border-top: 2px solid #5A1B1B; border-left: 2px solid #5A1B1B;
                }
                .dnd-markdown blockquote::after {
                    content: ''; position: absolute; bottom: -3px; right: -3px;
                    width: 14px; height: 14px;
                    border-bottom: 2px solid #5A1B1B; border-right: 2px solid #5A1B1B;
                }
                .dnd-markdown ul, .dnd-markdown ol { margin-bottom: 12px; padding-left: 20px; }
                .dnd-markdown table {
                    width: 100%; border-collapse: collapse; margin: 8px 0;
                    font-size: 13px;
                }
                .dnd-markdown th, .dnd-markdown td {
                    border: 1px solid #8b7355; padding: 8px 10px; text-align: left;
                }
                .dnd-markdown th {
                    background: rgba(90,27,27,0.08);
                    font-family: 'Cinzel', serif; font-size: 12px;
                    color: #5A1B1B; font-weight: 700;
                }
                .dnd-markdown td { background: rgba(255,255,255,0.3); }

                .audio-bar {
                    margin-top: 12px; padding: 10px 24px;
                    width: 100%; max-width: 1400px;
                    background: rgba(15,12,8,0.9);
                    border: 1px solid rgba(207,185,145,0.4);
                    border-radius: 6px; display: flex; align-items: center; justify-content: space-between;
                    box-shadow: 0 8px 16px rgba(0,0,0,0.4); z-index: 50;
                }
                .nav-group { display: flex; gap: 16px; align-items: center; }
                .nav-btn {
                    background: transparent; border: none;
                    color: #CFB991; font-family: 'Cinzel', serif;
                    font-weight: 700; font-size: 13px;
                    cursor: pointer; transition: all 0.2s; outline: none;
                }
                .nav-btn:hover:not(.nav-btn--disabled) { color: #fff; }
                .nav-btn--disabled { opacity: 0.3; cursor: default; }
            `}} />
        </div>
    );
}
